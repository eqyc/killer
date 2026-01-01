# 微服务 (services)

存放所有业务微服务，按业务域分组。

## 业务域分组

- `infrastructure/` - 基础设施域（系统管理、权限、审计等）
- `finance/` - 财务域（总账、应收应付、资产管理等）
- `procurement-ops/` - 采购运营域（采购、供应商管理等）
- `operations/` - 运营域（生产、质量、设备等）
- `logistics/` - 物流域（仓储、运输、库存等）
- `commercial/` - 商业域（销售、客户、定价等）
- `project-rd/` - 项目研发域（项目管理、研发等）
- `human-capital/` - 人力资本域（人事、薪酬、考勤等）

## 服务架构

每个服务采用 DDD 四层架构：
- `api/` - 接口层
- `application/` - 应用层
- `domain/` - 领域层
- `infrastructure/` - 基础设施层
