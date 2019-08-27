use super::*;
use super::DataServiceProvider;
use log::error;
impl DataServiceProvider{

    pub fn update_mnemonic(&self, mn: TbMnemonic) -> Result<(), String> {
        let mn_sql = "update Mnemonic set mnemonic=? where id=?;";
        let mut statement = self.db_hander.prepare(mn_sql).expect("sql statement is error!");
        statement.bind(1, mn.mnemonic.unwrap().as_str()).expect(" mnemonic  bind error");
        statement.bind(2, mn.id.unwrap().as_str()).expect("mn id bind error");

        match statement.next() {
            Ok(_state) => {
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn save_mnemonic_address(&mut self, mn: TbMnemonic, addr: TbAddress) -> Result<(), String> {
        let mn_sql = "insert into Mnemonic(id,mnemonic) values(?,?);";
        let address_sql = "insert into wallet.Address(mnemonic_id,chain_id,address,puk_key,status) values(?,?,?,?,?);";
        let digit_account_sql = "insert into wallet.Digit(address) values(?);";
        // TODO 增加事务的处理，这个的编码方式还需要修改 才能编译通过
        // TODO 根据链的地址种类 对应的填写代币账户信息
        match self.db_hander.prepare(mn_sql) {
            Ok(mut stat) => {
                stat.bind(1, mn.id.unwrap().as_str()).expect("save_mnemonic_address bind mn id error");
                stat.bind(2, mn.mnemonic.unwrap().as_str()).expect("save_mnemonic_address bind mnemonic error");
                match stat.next() {
                    Ok(_) => {
                        match self.db_hander.prepare(address_sql) {
                            Ok(mut address_stat) => {
                                address_stat.bind(1, addr.mnemonic_id.as_str()).expect("save_mnemonic_address bind addr.mnemonic_id ");
                                address_stat.bind(2, addr.chain_id as i64).expect("save_mnemonic_address bind addr.chain_id ");
                                address_stat.bind(3, addr.address.as_str()).expect("save_mnemonic_address bind addr.address ");
                                address_stat.bind(4, addr.pub_key.as_str()).expect("save_mnemonic_address bind addr.pub_key ");
                                address_stat.bind(5, addr.status as i64).expect("save_mnemonic_address  bind addr.status ");
                                match address_stat.next() {
                                    Ok(_) => {
                                        match self.db_hander.prepare(digit_account_sql) {
                                            Ok(mut digit_stat) => {
                                                digit_stat.bind(1, addr.address.as_str()).expect("save_mnemonic_address digit_stat bind addr.address ");
                                                digit_stat.next().expect("exec digit insert error");
                                                Ok(())
                                            }
                                            Err(e) => Err(e.to_string())
                                        }
                                    }
                                    Err(e) => Err(e.to_string())
                                }
                            }
                            Err(e) => Err(e.to_string())
                        }
                    }
                    Err(e) => Err(e.to_string())
                }
            }
            Err(e) => Err(e.to_string())
        }
    }

    //这个地方 定义成通用的对象查询功能
    pub fn query_by_mnemonic_id(&self, id: &str) -> Option<TbMnemonic> {
        let query_sql = "select * from Mnemonic where id = ?";
        let mut statement = self.db_hander.prepare(query_sql).unwrap();
        statement.bind(1, id).expect("query_by_mnemonic_id bind id");
        let mut cursor = statement.cursor();

        match cursor.next().unwrap() {
            Some(value) => {
                let mnemonic = TbMnemonic {
                    id: value[0].as_string().map(|str| String::from(str)),
                    full_name: value[1].as_string().map(|str| String::from(str)),
                    mnemonic: value[2].as_string().map(|str| String::from(str)),
                    selected: value[3].as_integer().map(|num| if num == 1 { true } else { false }),
                    status: value[4].as_integer(),
                    create_time: value[5].as_string().map(|str| String::from(str)),
                    update_time: value[6].as_string().map(|str| String::from(str)),
                };
                Some(mnemonic)
            }
            None => {
                None
            }
        }
    }

    //考虑修改为查询所有指定条件的对象
    pub fn query_selected_mnemonic(&self) -> Result<TbMnemonic, String> {
        //选中钱包 只有一个
        let sql = "select * from Mnemonic where selected=1;";

        self.db_hander.prepare(sql).map(|stmt| {
            let mut rows = stmt.cursor();
            let cursor = rows.next().unwrap().unwrap(); //必须存在一个指定的钱包
            let mnemonic = TbMnemonic {
                id: cursor[0].as_string().map(|str| String::from(str)),
                full_name: cursor[1].as_string().map(|str| String::from(str)),
                mnemonic: cursor[2].as_string().map(|str| String::from(str)),
                selected: cursor[3].as_integer().map(|num| if num == 1 { true } else { false }),
                status: cursor[4].as_integer(),
                create_time: cursor[5].as_string().map(|str| String::from(str)),
                update_time: cursor[6].as_string().map(|str| String::from(str)),
            };
            mnemonic
        }).map_err(|err| {
            error!("query error:{}", err.to_string());
            err.to_string()
        })
    }

    //当前该功能是返回所有的助记词
    pub fn get_mnemonics(&self) -> Vec<TbMnemonic> {
        let sql = "select * from Mnemonic WHERE status = 1;";

        let mut cursor = self.db_hander.prepare(sql).unwrap().cursor();
        let mut vec = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let mnemonic = TbMnemonic {
                id: row[0].as_string().map(|str| String::from(str)),
                full_name: row[1].as_string().map(|str| String::from(str)),
                mnemonic: row[2].as_string().map(|str| String::from(str)),
                selected: row[3].as_integer().map(|num| if num == 1 { true } else { false }),
                status: row[4].as_integer(),
                create_time: row[5].as_string().map(|str| String::from(str)),
                update_time: row[6].as_string().map(|str| String::from(str)),
            };
            vec.push(mnemonic)
        }
        vec
    }

    pub fn set_selected_mnemonic(&self, mn_id: &str) -> Result<(), String> {
        //需要先查询出那些助记词是被设置为当前选中，将其设置为取消选中，再将指定的id 设置为选中状态
        let sql = "UPDATE Mnemonic set selected = 0 where id in (select id from Mnemonic WHERE selected=1);";
        self.db_hander.execute(sql).expect("exec sql is error!");
        let set_select_sql = "update Mnemonic set selected = 1 where id =?;";
        self.db_hander.prepare(set_select_sql).map(|mut stat| {
            stat.bind(1, mn_id).expect("set_selected_mnemonic bind mn_id");
            stat.next().expect("exec set_selected_mnemonic error");
            ()
        }).map_err(|err| err.to_string())
        // self.db_hander.execute(set_select_sql, &[mn_id]).map(|_| ()).map_err(|err| err.to_string())
    }

    pub fn del_mnemonic(&self, mn_id: &str) -> Result<(), String> {
        let sql = "DELETE from Mnemonic WHERE id = ?; ";
        let update_address = "UPDATE Address set status = 0 WHERE mnemonic_id =?;";
        match self.db_hander.prepare(sql) {
            Ok(mut stat) => {
                stat.bind(1, mn_id).expect("del_mnemonic bind mn_id");
                stat.next().expect("exec del_mnemonic");
                match self.db_hander.prepare(update_address) {
                    Ok(mut stat) => {
                        stat.bind(1, mn_id).expect("del_mnemonic update_address bind mn_id");
                        stat.next().expect("exec del_mnemonic update_address");
                        Ok(())
                    }
                    Err(e) => {
                        Err(e.to_string())
                    }
                }
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn rename_mnemonic(&self, mn_id: &str, mn_name: &str) -> Result<(), String> {
        let sql = "UPDATE Mnemonic set fullname = ? WHERE id=?;";
        match self.db_hander.prepare(sql) {
            Ok(mut stat) => {
                let bind_mn_name = stat.bind(1, mn_name).map_err(|e| format!("rename_mnemonic bind mn name,{}", e.to_string()));
                if bind_mn_name.is_err() {
                    return Err(bind_mn_name.unwrap_err());
                }
                let bind_mn_id = stat.bind(2, mn_id).map_err(|e| format!("rename_mnemonic bind mn id,{}", e.to_string()));
                if bind_mn_id.is_err() {
                    return Err(bind_mn_id.unwrap_err());
                }
                let exec = stat.next().map_err(|e| format!("exec rename_mnemonic,{}", e.to_string()));
                if exec.is_err() {
                    return Err(exec.unwrap_err());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn display_mnemonic_list(&self) -> Result<Vec<TbWallet>, String> {
        let all_mn = "select a.id as wallet_id,a.fullname as wallet_name,b.id as chain_id,d.address,b.address as chain_address,a.selected,b.type as chian_type,d.id as digit_id,d.contract_address,d.short_name,d.full_name,d.balance,d.selected as isvisible,d.decimals,d.url_img
 from Mnemonic a,wallet.Chain b,wallet.Address c,wallet.Digit d where a.id=c.mnemonic_id and c.chain_id = b.id and c.address=d.address and a.status =1 and c.status =1;";

        let mut cursor = self.db_hander.prepare(all_mn).unwrap().cursor();
        let mut tbwallets = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            println!("query wallet_id {:?},wallet_name:{:?}", row[0].as_string(),row[1].as_string());
            let tbwallet = TbWallet {
                wallet_id: row[0].as_string().map(|str| String::from(str)),
                wallet_name: row[1].as_string().map(|str| String::from(str)),
                chain_id: row[2].as_integer(),
                address: row[3].as_string().map(|str| String::from(str)),
                chain_address: row[4].as_string().map(|str| String::from(str)),
                selected: row[5].as_integer().map(|num| if num == 1 { true } else { false }),
                chain_type: row[6].as_integer(),
                digit_id: row[7].as_integer(),
                contract_address: row[8].as_string().map(|str| String::from(str)),
                short_name: row[9].as_string().map(|str| String::from(str)),
                full_name: row[10].as_string().map(|str| String::from(str)),
                balance: row[11].as_string().map(|str| String::from(str)),
                isvisible: row[12].as_integer().map(|num| if num == 1 { true } else { false }),
                decimals: row[13].as_integer(),
                url_img: row[14].as_string().map(|str| String::from(str)),
            };
            tbwallets.push(tbwallet);
        }
        Ok(tbwallets)
    }
}