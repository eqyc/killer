# HR Service (PA)

核心人力资源服务，负责员工基础信息管理。包括员工档案、组织架构、考勤管理、假期管理和员工自助服务。

## 服务职责

| 模块 | 职责 |
|------|------|
| 员工管理 | 入职、转正、离职 |
| 组织管理 | 部门、岗位、职级 |
| 考勤管理 | 打卡、加班、外勤 |
| 假期管理 | 年假、病假、调休 |
| 员工自助 | 个人信息、证明开具 |

## 核心聚合根

| 聚合根 | 说明 |
|--------|------|
| `Employee` | 员工 |
| `Organization` | 组织 |
| `Position` | 岗位 |
| `Attendance` | 考勤记录 |
| `LeaveRequest` | 请假申请 |
| `EmploymentContract` | 劳动合同 |

## 领域事件

| 事件 | 触发时机 |
|------|----------|
| `EmployeeOnboarded` | 员工入职 |
| `EmployeePromoted` | 员工晋升 |
| `EmployeeTransferred` | 员工调动 |
| `EmployeeOffboarded` | 员工离职 |
| `LeaveApproved` | 请假审批 |
| `AttendanceRecorded` | 考勤记录 |

## 依赖的主数据

| 主数据 | 来源 | 用途 |
|--------|------|------|
| `CompanyCode` | organizational-units | 公司 |
| `CostCenter` | cost-center | 成本中心 |
| `Department` | 本服务 | 部门 |

## 端口配置

| 端口 | 用途 |
|------|------|
| 8080 | HTTP API |
| 50051 | gRPC |
| 9090 | Prometheus Metrics |
