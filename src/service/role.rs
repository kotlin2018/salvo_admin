use rbatis::crud::CRUDMut;
use rbatis::DateTimeNative;
use rbatis::executor::RBatisTxExecutor;
use crate::entity::user::UserEntity;
use crate::entity::user_role::UserRoleEntity;
use crate::init_rbatis;

pub struct Role{}

impl Role {

    /// 编辑用户-角色关系表
    pub async fn edit_user_role(&mut tx: RBatisTxExecutor,user_id: &str,role_ids: Vec<String>,created_by: &str){
        let _ = role_ids.
            clone() // 这里不能消费掉 role_ids,后续还要使用 role_ids，因此先克隆一个 role_ids 的副本。
            .iter()// 迭代 role_ids，获取它中的每一个元素
            .map(|x:&String,tx: &mut RBatisTxExecutor| {     // 处理迭代出的单个元素
                let user = UserRoleEntity {
                    id: Some(scru128::scru128().to_string()),
                    user_id: Some(user_id.to_string()),
                    role_id: Some(x.to_string()),
                    created_by: Some(created_by.to_string()),
                    created_at: Some(DateTimeNative::now())
                };
                tx.save::<UserRoleEntity>(&user,&[]).await.expect("保存失败");
                // 更新用户的角色字段
            });
    }

    // 删除原有的角色信息，再添加新的角色信息

}