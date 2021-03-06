pub fn get_cashbox_wallet_detail_sql() -> &'static str {
    r#"
    PRAGMA foreign_keys = 'off';
    BEGIN;
    DROP TABLE IF EXISTS [main].[Address];
    CREATE TABLE [main].[Address](
    [address_id] VARCHAR(64) PRIMARY KEY NOT NULL,
    [wallet_id] VARCHAR(64) NOT NULL,
    [chain_id] INT NOT NULL,
    [address] VARCHAR(64) NOT NULL,
    [puk_key] VARCHAR(128) NOT NULL,
    [status] INT NOT NULL  DEFAULT 1,
    [is_visible] VARCHAR(1)  NOT NULL DEFAULT 1,
    [create_time] timestamp NOT NULL DEFAULT (strftime('%s','now'))
    );

    DROP TABLE IF EXISTS [main].[Chain];
    CREATE TABLE [main].[Chain](
    [id] INTEGER PRIMARY KEY NOT NULL,
    [type] INT,
    [short_name] VARCHAR(32),
    [full_name] VARCHAR(64),
    [domain] VARCHAR(128),
    [group_name] VARCHAR(32),
    [next_id] INT,
    [selected] VARCHAR(1) NOT NULL DEFAULT 1,
    [status] INT NOT NULL  DEFAULT 1,
    [more_property] VARCHAR(1),
    [create_time] timestamp NOT NULL DEFAULT (strftime('%s','now'))
    );

   DROP TABLE IF EXISTS [main].[DefaultDigitBase];
    CREATE TABLE [main].[DefaultDigitBase](
    [id] VARCHAR(40) PRIMARY KEY NOT NULL,
    [contract_address] VARCHAR(64),
	[chain_type] INT  NOT NULL,
    [short_name] VARCHAR(32),
    [full_name] VARCHAR(32),
    [next_id] INT,
    [url_img] VARCHAR(1024),
    [group_name] VARCHAR(32),
    [is_visible] VARCHAR(1)  NOT NULL DEFAULT 1,
    [decimals] INT,
	[unit]  VARCHAR(32),
	[is_basic] VARCHAR(1) NOT NULL  DEFAULT 0,
	[is_default] VARCHAR(1) NOT NULL  DEFAULT 0,
    [status] INT NOT NULL  DEFAULT 0,
    [CREATED_TIME] timestamp NOT NULL DEFAULT (strftime('%s','now'))
    );


    DROP TABLE IF EXISTS [main].[DigitUseDetail];
    CREATE TABLE [main].[DigitUseDetail](
    [digit_id] VARCHAR (40) ,
    [address_id] VARCHAR(64) ,
	[balance] VARCHAR(32) NOT NULL DEFAULT 0,
    [is_visible] VARCHAR(1)  NOT NULL DEFAULT 1,
    [status] INT NOT NULL DEFAULT 1,
    [CREATED_TIME] timestamp NOT NULL DEFAULT (strftime('%s','now')),
    [UPDATED_TIME] timestamp NOT NULL DEFAULT (strftime('%s','now')),
	primary key(digit_id,address_id));

    DROP TABLE IF EXISTS [main].[DigitBase];
     CREATE TABLE [main].[DigitBase] (
        [id]  VARCHAR (40),
        [chain_type]    INT  NOT NULL,
        [contract]      VARCHAR (64),
        [accept_id]     VARCHAR (32),
        [symbol]        VARCHAR (32),
        [name]  VARCHAR (32),
        [publisher]     VARCHAR (32),
        [project]       VARCHAR (32),
        [logo_url]      VARCHAR (1024),
        [logo_bytes]    VARCHAR (3072),
        [decimal]       INT,
        [gas_limit]     INT,
        [mark]  VARCHAR (512),
        [status]        INT NOT NULL DEFAULT 1,
        [is_auth]    VARCHAR (1) NOT NULL DEFAULT 0,
        [is_visible]    VARCHAR (1) NOT NULL DEFAULT 1,
        [CREATED_TIME]  timestamp NOT NULL DEFAULT (strftime('%s','now')),
        [UPDATED_TIME]  timestamp NOT NULL DEFAULT (strftime('%s','now')),
        [version]       INT,
        PRIMARY KEY(`id`)
);

    DROP TABLE IF EXISTS [main].[TransferRecord];
    CREATE TABLE [main].[TransferRecord](
    [tx_hash]  VARCHAR(72) PRIMARY KEY NOT NULL,
    [block_hash] VARCHAR(72),
    [chain_id] INT,
    [account] VARCHAR(48),
    [tx_index] INT,
    [tx_from] VARCHAR(48),
    [tx_to] VARCHAR(48),
    [amount] VARCHAR(32),
    [unit] VARCHAR(32),
    [status] int,
    [tx_timestamp] timestamp NOT NULL,
    [CREATED_TIME] timestamp NOT NULL DEFAULT (strftime('%s','now'))
);

    DROP TABLE IF EXISTS [main].[AccountInfoSyncProg];
    CREATE TABLE [main].[AccountInfoSyncProg](
    [account] VARCHAR(48) PRIMARY KEY NOT NULL,
    [chain_type] INT,
    [block_num] INT,
    [block_hash] VARCHAR(72),
    [update_time] timestamp NOT NULL DEFAULT (strftime('%s','now'))
);

     create trigger update_digit_detail_trigger after update on DigitUseDetail
      begin
        update DigitUseDetail set UPDATED_TIME = strftime('%s','now') where digit_id = new.digit_id and address_id = new.address_id ;
      end;

    insert into Chain(id,short_name,full_name,type,domain,selected) Values(1,'BTC',"bitcoin",1,"",1);
    insert into Chain(id,short_name,full_name,type,domain,selected) Values(2,'BTC TEST',"bitcoin test",2,"",1);
    insert into Chain(id,short_name,full_name,type,domain) Values(3,'ETH',"ethereum",3,"");
    insert into Chain(id,short_name,full_name,type,domain) Values(4,'ETH TEST',"ethereum test",4,"");
    insert into Chain(id,short_name,full_name,type,domain) Values(5,'EEE',"eee",5,"");
    insert into Chain(id,short_name,full_name,type,domain) Values(6,'EEE TEST',"eee test",6,"");

    COMMIT;
    PRAGMA foreign_keys = 'on';
    "#
}

pub fn get_cashbox_wallet_sql() -> &'static str {
    r#"
        PRAGMA foreign_keys = 'off';
        BEGIN;
        DROP TABLE IF EXISTS [main].[Wallet];
        CREATE TABLE [main].[Wallet](
          [wallet_id] VARCHAR(64) PRIMARY KEY NOT NULL,
          [mn_digest] VARCHAR(64) NOT NULL,
          [fullname] VARCHAR(32),
          [mnemonic] VARCHAR(3072),
          [wallet_type] INT NOT NULL  DEFAULT 1,
          [selected] VARCHAR(1),
          [status] INT NOT NULL  DEFAULT 1,
          [display_chain_id] INT NOT NULL,
          [create_time] timestamp NOT NULL DEFAULT (strftime('%s','now')),
          [update_time] timestamp NOT NULL DEFAULT (strftime('%s','now')));

          create trigger update_time_trigger after update on Wallet
          begin
            update Wallet set update_time = strftime('%s','now') where wallet_id = new.wallet_id;
          end;
        COMMIT;
        PRAGMA foreign_keys = 'on';
    "#
}
