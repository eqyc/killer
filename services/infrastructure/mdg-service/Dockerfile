# =============================================================================
# 构建阶段
# =============================================================================
FROM rust:1.75-slim as builder

# 安装依赖
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./

# 复制源代码
COPY src ./src
COPY proto ./proto
COPY config ./config

# 构建发布版本
RUN cargo build --release --bin mdg-service

# =============================================================================
# 运行阶段
# =============================================================================
FROM debian:bookworm-slim

# 安装运行时依赖
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# 创建非 root 用户
RUN useradd -m -u 1000 mdg

WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/mdg-service /usr/local/bin/mdg-service

# 复制配置文件
COPY --from=builder /app/config /etc/mdg/config

# 切换到非 root 用户
USER mdg

# 暴露端口
EXPOSE 50051 8080

# 健康检查
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# 启动服务
CMD ["mdg-service", "--config", "/etc/mdg/config/mdg.yaml"]
