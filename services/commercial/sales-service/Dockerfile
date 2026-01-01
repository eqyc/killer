# ============================================================================
# Sales Service Dockerfile
# 多阶段构建，优化镜像大小
# ============================================================================

# ---- Builder Stage ----
FROM rust:1.85-bookworm AS builder

WORKDIR /app

# 安装构建依赖
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    libprotobuf-dev \
    cmake \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

# 复制 workspace 配置和依赖
COPY Cargo.toml Cargo.lock ./
COPY libs ./libs
COPY proto ./proto
COPY services/commercial/sales-service ./services/commercial/sales-service

# 构建 release 版本
RUN cargo build --release -p killer-sales-service

# ---- Runtime Stage ----
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 从 builder 复制二进制文件
COPY --from=builder /app/target/release/sales-service /app/sales-service

# 复制配置文件
COPY services/commercial/sales-service/config /app/config

# 创建非 root 用户
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app
USER appuser

# 暴露端口
EXPOSE 8080 50051 9090

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动命令
ENTRYPOINT ["/app/sales-service"]
CMD ["--config", "/app/config/production.toml"]
