
/// # 干员部分代码
/// * 干员技能绑定
mod char;

/// # 基建模拟器部分
/// * 干员进驻加成
/// * 干员苏亮加成
/// * 基建自身加成
/// * 控制中枢加成
/// * 其他干员加成
mod simulation;

/// #干员技能部分
/// * 技能生效效果
/// * 技能与时间关系
/// * 技能作用区域
/// * 待补充
mod skill;

/// # 通用的traits
/// * 技能生效trait
/// * 干员容器
/// * 待补充
mod util;

/// # 核心算法部分
/// 1. 单个生产线
/// 2. 单个贸易站
/// 3. 生产线与贸易站组合
/// 4. 基建全联动
mod core;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
