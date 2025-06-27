# syntax=docker/dockerfile:1
FROM python:3.12-slim

WORKDIR /app

# Copy project files
COPY . .

# Tạo thư mục logs để tránh lỗi khi ghi log
RUN mkdir -p logs

# Install dependencies
RUN pip install --upgrade pip \
    && pip install --no-cache-dir fastapi uvicorn pydantic

# Install any local packages if needed
# RUN pip install -e .

EXPOSE 8000

CMD ["python", "src/server.py"]
