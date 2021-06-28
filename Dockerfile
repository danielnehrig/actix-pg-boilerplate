FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/customer-storage-service .
CMD ["/app/customer-storage-service"]
