import json
import os
from datetime import datetime, timezone

APP_NAME = os.getenv("APP_NAME", "hello-lambda")

def handler(event, context):
    """
    Ejemplo mínimo de AWS Lambda en Python.
    Similar al mostrado en el video: devuelve un mensaje JSON
    y demuestra cómo se estructura una respuesta en formato HTTP.
    """
    now = datetime.now(timezone.utc).isoformat()
    request_id = getattr(context, "aws_request_id", "local-test")

    body = {
        "app": APP_NAME,
        "message": "Hola desde AWS Lambda 👋",
        "event_keys": list(event.keys()) if isinstance(event, dict) else [],
        "request_id": request_id,
        "timestamp_utc": now
    }

    return {
        "statusCode": 200,
        "headers": {"Content-Type": "application/json"},
        "body": json.dumps(body, ensure_ascii=False)
    }

# Prueba local (python handler.py)
if _name_ == "__main__":
    fake_ctx = type("ctx", (), {"aws_request_id": "dev-local"})()
    print(handler({"ping": True}, fake_ctx))
