# bevy笔记

## ECS

> 实体(Entity) 是指向一群 组件(Component) 的唯一 事物(things), 然后使用 系统(System) 处理其过程.

> ECS模式鼓励清晰, (解耦)分离的设计, 强制使你将数据和逻辑分解为核心组件. 通过优化内存访问模式并简化并行性, 有助于使你的代码更快.


个人理解: 

C -> E -> S, 组件 是基本的单元, 实体 由各个组件合成, 系统 通过组件 间接 控制 实体.


app 运行的整个周期 就是: 开始运行 -> update -> 结束运行, bevy 把这个过程用 Stage 来管理.

    CoreStage (StageLabel):
        Startup         在 app 启动的开始 运行一次
        First           在 其它的 app stages 之前 运行
        PreEvent        在 EVENT 之前 运行
        Event           在 UPDATE 之前 运行
        PreUpdate       在 UPDATE 之前 运行, 负责执行 setup
        Update          负责做大多数 app 逻辑, Systems 应该被注册在这里
        PostUpdate      在 UPDATE 之后 运行, 负责处理 UPDATE 的结果
        Last            在其它所有的 app stages 之后 运行


事件的处理:
    添加事件, 发布事件, 接收事件, 更新关联资源


一个最小系统的设计:

    . 添加一个 LogPlugin, 用于 处理 log 输出

    . 添加一个 Time 资源, 再添加一个 TimeSystem, 用于更新 程序的 Time 资源

    . 添加一个
    
    . 事件 -- 定时器事件, 5s 周期性变化
        当 定时器 时间达到时, 改变 sprite 颜色

    . 添加 点击
