#!/usr/bin/env python3
"""Send email notification when a pending question exists."""

import smtplib
import sys
import os
from email.mime.text import MIMEText
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
    for k in ("RS_EMAIL", "RS_EMAIL_PASSWORD", "RS_SMTP_HOST", "RS_SMTP_PORT"):
        config[k] = os.environ.get(k, config.get(k, ""))
    return config


def send(subject, body, config):
    msg = MIMEText(body, "plain", "utf-8")
    msg["From"] = config["RS_EMAIL"]
    msg["To"] = config["RS_EMAIL"]
    msg["Subject"] = subject

    with smtplib.SMTP(config["RS_SMTP_HOST"], int(config["RS_SMTP_PORT"])) as s:
        s.starttls()
        s.login(config["RS_EMAIL"], config["RS_EMAIL_PASSWORD"])
        s.send_message(msg)


if __name__ == "__main__":
    project = sys.argv[1] if len(sys.argv) > 1 else "."
    question_file = Path(project) / "pending_question.md"

    if not question_file.exists() or question_file.stat().st_size == 0:
        print("No pending question.")
        sys.exit(0)

    body = question_file.read_text()
    project_name = Path(project).resolve().name
    cfg = load_config()
    send(f"[rs] {project_name}: decision needed", body, cfg)
    print(f"Email sent to {cfg['RS_EMAIL']}")
