#!/usr/bin/env python3
"""Poll IMAP for reply to a pending question. Write reply to reply.md."""

import imaplib
import email
import time
import sys
import os
import re
from email.header import decode_header
from pathlib import Path


def load_config():
    config = {}
    env_file = Path(__file__).parent / "config.env"
    if env_file.exists():
        for line in env_file.read_text().splitlines():
            line = line.strip()
            if line and not line.startswith("#") and "=" in line:
                k, v = line.split("=", 1)
                config[k.strip()] = v.strip().strip('"')
    for k in (
        "RS_EMAIL",
        "RS_EMAIL_PASSWORD",
        "RS_IMAP_HOST",
        "RS_IMAP_PORT",
        "RS_POLL_INTERVAL",
    ):
        config[k] = os.environ.get(k, config.get(k, ""))
    return config


def get_text(msg):
    if msg.is_multipart():
        for part in msg.walk():
            if part.get_content_type() == "text/plain":
                payload = part.get_payload(decode=True)
                if payload:
                    return payload.decode("utf-8", errors="replace")
    else:
        payload = msg.get_payload(decode=True)
        if payload:
            return payload.decode("utf-8", errors="replace")
    return ""


def strip_reply_quotes(text):
    lines = text.splitlines()
    clean = []
    for line in lines:
        if line.startswith(">") or line.startswith("On ") and "wrote:" in line:
            break
        clean.append(line)
    return "\n".join(clean).strip()


def poll_once(config):
    with imaplib.IMAP4_SSL(config["RS_IMAP_HOST"], int(config["RS_IMAP_PORT"])) as M:
        M.login(config["RS_EMAIL"], config["RS_EMAIL_PASSWORD"])
        M.select("INBOX")
        _, data = M.search(
            None, "UNSEEN", "SUBJECT", '"[rs]"', "FROM", f'"{config["RS_EMAIL"]}"'
        )
        ids = data[0].split()
        if not ids:
            return None
        latest = ids[-1]
        _, msg_data = M.fetch(latest, "(RFC822)")
        raw = msg_data[0][1]
        msg = email.message_from_bytes(raw)
        body = get_text(msg)
        reply = strip_reply_quotes(body)
        M.store(latest, "+FLAGS", "\\Seen")
        return reply


if __name__ == "__main__":
    project = sys.argv[1] if len(sys.argv) > 1 else "."
    cfg = load_config()
    interval = int(cfg.get("RS_POLL_INTERVAL", "60"))
    reply_file = Path(project) / "reply.md"

    print(f"Polling {cfg['RS_EMAIL']} every {interval}s for [rs] replies...")
    while True:
        reply = poll_once(cfg)
        if reply:
            reply_file.write_text(reply)
            print(f"Reply received, written to {reply_file}")
            break
        time.sleep(interval)
