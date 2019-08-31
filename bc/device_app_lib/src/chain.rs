use jni::JNIEnv;
use jni::objects::{JObject, JValue};
use wallets::model::{EeeChain,BtcChain,EthChain};

pub fn get_eee_chain_obj<'a, 'b>(env:  &'a JNIEnv<'b>,eee_chain:EeeChain)->JObject<'a>{
    let eee_digit_list_class = env.find_class("java/util/ArrayList").expect("find chain type is error");
    let eee_digit_class = env.find_class("info/scry/wallet_manager/NativeLib$EeeDigit").expect("Digit class not found");
    let eee_chain_class = env.find_class("info/scry/wallet_manager/NativeLib$EeeChain").expect("chain class find error");
    let chain_class_obj = env.alloc_object(eee_chain_class).expect("create chain_class instance is error!");
    env.set_field(chain_class_obj, "status", "I", JValue::Int(eee_chain.status as i32)).expect("set status value is error!");
    let chain_id_str = format!("{}", eee_chain.chain_id);

    env.set_field(chain_class_obj, "chainId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(chain_id_str).unwrap()))).expect("set chainId value is error!");
    env.set_field(chain_class_obj, "walletId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(eee_chain.wallet_id).unwrap()))).expect("set digitId value is error!");

    if eee_chain.chain_address.is_some() {
        env.set_field(chain_class_obj, "chainAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(eee_chain.chain_address.unwrap()).unwrap()))).expect("set status value is error!");
    }

    if eee_chain.is_visible.is_some() {
        let visible = eee_chain.is_visible.unwrap();
        env.set_field(chain_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set digitId value is error!");
    }
    if eee_chain.chain_type.is_some() {
        let chain_type = eee_chain.chain_type.unwrap();
        env.set_field(chain_class_obj, "chainType", "I", JValue::Int(chain_type as i32)).expect("set digitId value is error!");
    }
    //每一条链下存在多个代币，需要使用List来存储
    let digit_list_obj = env.alloc_object(eee_digit_list_class).expect("create digit_list_obj instance is error!");
    env.call_method(digit_list_obj, "<init>", "()V", &[]).expect("chain chain obj init method is exec");

    for digit in eee_chain.digit_list {
        //实例化 chain
        let digit_class_obj = env.alloc_object(eee_digit_class).expect("create chain instance is error!");
        //设置digit 属性
        env.set_field(digit_class_obj, "status", "I", JValue::Int(digit.status as i32)).expect("set status value is error!");

        env.set_field(digit_class_obj, "digitId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.digit_id).unwrap()))).expect("set digitId value is error!");
        if digit.address.is_some() {
            env.set_field(digit_class_obj, "address", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.address.unwrap()).unwrap()))).expect("set address value is error!");
        }
        if digit.contract_address.is_some() {
            env.set_field(digit_class_obj, "contractAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.contract_address.unwrap()).unwrap()))).expect("set contractAddress value is error!");
        }
        if digit.shortname.is_some() {
            env.set_field(digit_class_obj, "shortName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.shortname.unwrap()).unwrap()))).expect("set shortName value is error!");
        }
        if digit.fullname.is_some() {
            env.set_field(digit_class_obj, "fullName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.fullname.unwrap()).unwrap()))).expect("set fullName value is error!");
        }
        if digit.balance.is_some() {
            env.set_field(digit_class_obj, "balance", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.balance.unwrap()).unwrap()))).expect("set balance value is error!");
        }
        if digit.is_visible.is_some() {
            let visible = digit.is_visible.unwrap();
            env.set_field(digit_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set isVisible value is error!");
        }
        if digit.decimal.is_some() {
            let decimal = digit.decimal.unwrap();
            env.set_field(digit_class_obj, "decimal", "I", JValue::Int(decimal as i32)).expect("set decimal value is error!");
        }

        if digit.decimal.is_some() {
            env.set_field(digit_class_obj, "imgUrl", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.imgurl.unwrap()).unwrap()))).expect("set imgUrl value is error!");
        }

        env.call_method(digit_list_obj, "add", "(Ljava/lang/Object;)Z", &[digit_class_obj.into()]).expect("add chain instance is fail");
    }
    env.set_field(chain_class_obj,"digitList","Ljava/util/List;",JValue::Object(digit_list_obj)).expect("set digitList");
    chain_class_obj
}


pub fn get_eth_chain_obj<'a, 'b>(env:  &'a JNIEnv<'b>,eth_chain:EthChain)->JObject<'a>{
    let eth_digit_list_class = env.find_class("java/util/ArrayList").expect("find chain type is error");
    let eth_digit_class = env.find_class("info/scry/wallet_manager/NativeLib$EthDigit").expect("Digit class not found");
    let eth_chain_class = env.find_class("info/scry/wallet_manager/NativeLib$EthChain").expect("chain class find error");
    let chain_class_obj = env.alloc_object(eth_chain_class).expect("create chain_class instance is error!");
    env.set_field(chain_class_obj, "status", "I", JValue::Int(eth_chain.status as i32)).expect("set status value is error!");
    let chain_id_str = format!("{}", eth_chain.chain_id);

    env.set_field(chain_class_obj, "chainId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(chain_id_str).unwrap()))).expect("set chainId value is error!");
    env.set_field(chain_class_obj, "walletId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(eth_chain.wallet_id).unwrap()))).expect("set digitId value is error!");

    if eth_chain.chain_address.is_some() {
        env.set_field(chain_class_obj, "chainAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(eth_chain.chain_address.unwrap()).unwrap()))).expect("set status value is error!");
    }

    if eth_chain.is_visible.is_some() {
        let visible = eth_chain.is_visible.unwrap();
        env.set_field(chain_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set digitId value is error!");
    }
    if eth_chain.chain_type.is_some() {
        let chain_type = eth_chain.chain_type.unwrap();
        env.set_field(chain_class_obj, "chainType", "I", JValue::Int(chain_type as i32)).expect("set digitId value is error!");
    }
    //每一条链下存在多个代币，需要使用List来存储
    let digit_list_obj = env.alloc_object(eth_digit_list_class).expect("create digit_list_obj instance is error!");
    env.call_method(digit_list_obj, "<init>", "()V", &[]).expect("chain chain obj init method is exec");

    for digit in eth_chain.digit_list {
        //实例化 chain
        let digit_class_obj = env.alloc_object(eth_digit_class).expect("create chain instance is error!");
        //设置digit 属性
        env.set_field(digit_class_obj, "status", "I", JValue::Int(digit.status as i32)).expect("set status value is error!");

        env.set_field(digit_class_obj, "digitId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.digit_id).unwrap()))).expect("set digitId value is error!");
        if digit.address.is_some() {
            env.set_field(digit_class_obj, "address", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.address.unwrap()).unwrap()))).expect("set address value is error!");
        }
        if digit.contract_address.is_some() {
            env.set_field(digit_class_obj, "contractAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.contract_address.unwrap()).unwrap()))).expect("set contractAddress value is error!");
        }
        if digit.shortname.is_some() {
            env.set_field(digit_class_obj, "shortName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.shortname.unwrap()).unwrap()))).expect("set shortName value is error!");
        }
        if digit.fullname.is_some() {
            env.set_field(digit_class_obj, "fullName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.fullname.unwrap()).unwrap()))).expect("set fullName value is error!");
        }
        if digit.balance.is_some() {
            env.set_field(digit_class_obj, "balance", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.balance.unwrap()).unwrap()))).expect("set balance value is error!");
        }
        if digit.is_visible.is_some() {
            let visible = digit.is_visible.unwrap();
            env.set_field(digit_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set isVisible value is error!");
        }
        if digit.decimal.is_some() {
            let decimal = digit.decimal.unwrap();
            env.set_field(digit_class_obj, "decimal", "I", JValue::Int(decimal as i32)).expect("set decimal value is error!");
        }

        if digit.decimal.is_some() {
            env.set_field(digit_class_obj, "imgUrl", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.imgurl.unwrap()).unwrap()))).expect("set imgUrl value is error!");
        }

        env.call_method(digit_list_obj, "add", "(Ljava/lang/Object;)Z", &[digit_class_obj.into()]).expect("add chain instance is fail");
    }
    env.set_field(chain_class_obj,"digitList","Ljava/util/List;",JValue::Object(digit_list_obj)).expect("set digitList");
    chain_class_obj
}

pub fn get_btc_chain_obj<'a, 'b>(env:  &'a JNIEnv<'b>,btc_chain:BtcChain)->JObject<'a>{
    let btc_digit_list_class = env.find_class("java/util/ArrayList").expect("find chain type is error");
    let btc_digit_class = env.find_class("info/scry/wallet_manager/NativeLib$BtcDigit").expect("Digit class not found");
    let btc_chain_class = env.find_class("info/scry/wallet_manager/NativeLib$BtcChain").expect("chain class find error");
    let chain_class_obj = env.alloc_object(btc_chain_class).expect("create chain_class instance is error!");
    env.set_field(chain_class_obj, "status", "I", JValue::Int(btc_chain.status as i32)).expect("set status value is error!");

    let chain_id_str = format!("{}", btc_chain.chain_id);
    env.set_field(chain_class_obj, "chainId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(chain_id_str).unwrap()))).expect("set chainId value is error!");
    env.set_field(chain_class_obj, "walletId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(btc_chain.wallet_id).unwrap()))).expect("set digitId value is error!");

    if btc_chain.chain_address.is_some() {
        env.set_field(chain_class_obj, "chainAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(btc_chain.chain_address.unwrap()).unwrap()))).expect("set status value is error!");
    }

    if btc_chain.is_visible.is_some() {
        let visible = btc_chain.is_visible.unwrap();
        env.set_field(chain_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set digitId value is error!");
    }
    if btc_chain.chain_type.is_some() {
        let chain_type = btc_chain.chain_type.unwrap();
        env.set_field(chain_class_obj, "chainType", "I", JValue::Int(chain_type as i32)).expect("set digitId value is error!");
    }
    //每一条链下存在多个代币，需要使用List来存储
    let digit_list_obj = env.alloc_object(btc_digit_list_class).expect("create digit_list_obj instance is error!");
    env.call_method(digit_list_obj, "<init>", "()V", &[]).expect("chain chain obj init method is exec");

    for digit in btc_chain.digit_list {
        //实例化 chain
        let digit_class_obj = env.alloc_object(btc_digit_class).expect("create chain instance is error!");
        //设置digit 属性
        env.set_field(digit_class_obj, "status", "I", JValue::Int(digit.status as i32)).expect("set status value is error!");

        env.set_field(digit_class_obj, "digitId", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.digit_id).unwrap()))).expect("set digitId value is error!");
        if digit.address.is_some() {
            env.set_field(digit_class_obj, "address", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.address.unwrap()).unwrap()))).expect("set address value is error!");
        }
        if digit.contract_address.is_some() {
            env.set_field(digit_class_obj, "contractAddress", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.contract_address.unwrap()).unwrap()))).expect("set contractAddress value is error!");
        }
        if digit.shortname.is_some() {
            env.set_field(digit_class_obj, "shortName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.shortname.unwrap()).unwrap()))).expect("set shortName value is error!");
        }
        if digit.fullname.is_some() {
            env.set_field(digit_class_obj, "fullName", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.fullname.unwrap()).unwrap()))).expect("set fullName value is error!");
        }
        if digit.balance.is_some() {
            env.set_field(digit_class_obj, "balance", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.balance.unwrap()).unwrap()))).expect("set balance value is error!");
        }
        if digit.is_visible.is_some() {
            let visible = digit.is_visible.unwrap();
            env.set_field(digit_class_obj, "isVisible", "Z", JValue::Bool(visible as u8)).expect("set isVisible value is error!");
        }
        if digit.decimal.is_some() {
            let decimal = digit.decimal.unwrap();
            env.set_field(digit_class_obj, "decimal", "I", JValue::Int(decimal as i32)).expect("set decimal value is error!");
        }

        if digit.decimal.is_some() {
            env.set_field(digit_class_obj, "imgUrl", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digit.imgurl.unwrap()).unwrap()))).expect("set imgUrl value is error!");
        }

        env.call_method(digit_list_obj, "add", "(Ljava/lang/Object;)Z", &[digit_class_obj.into()]).expect("add chain instance is fail");
    }
    env.set_field(chain_class_obj,"digitList","Ljava/util/List;",JValue::Object(digit_list_obj)).expect("set digitList");
    chain_class_obj
}