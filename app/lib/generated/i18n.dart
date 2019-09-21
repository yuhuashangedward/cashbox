import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';

// ignore_for_file: non_constant_identifier_names
// ignore_for_file: camel_case_types
// ignore_for_file: prefer_single_quotes

// This file is automatically generated. DO NOT EDIT, all your changes would be lost.
class S implements WidgetsLocalizations {
  const S();

  static S current;

  static const GeneratedLocalizationsDelegate delegate =
    GeneratedLocalizationsDelegate();

  static S of(BuildContext context) => Localizations.of<S>(context, S);

  @override
  TextDirection get textDirection => TextDirection.ltr;

  String get about_us_title => "关于我们";
  String get add_wallet => "添加钱包";
  String get advice_pwd_format => "建议大于8位，英文、数字混合";
  String get agree_service_prefix => "我已仔细阅读并同意";
  String get app_title => "App标题";
  String get backup_mnemonic => "备份助记词";
  String get chain_address_info => "链地址信息";
  String get change_another_group => "换一组";
  String get choose_multi_chain => "选择创建链";
  String get create_test_wallet => "创建测试钱包";
  String get create_wallet => "创建钱包";
  String get dapp => "Dapp";
  String get eee_chain_test => "EEE_TEST";
  String get failure_create_test_wallet => "测试钱包创建失败，请检查你输入的数据是否正确";
  String get failure_to_change_wallet => "钱包切换失败，请重新打开钱包，尝试切换";
  String get failure_to_load_data_pls_retry => "数据加载出错了，请尝试重新加载!~";
  String get import_wallet => "导入钱包";
  String get judge_the_difference_between_two_wallet => "注意：此测试钱包里面能使用的,都是测试链上的代币。      请区分与正式链的差别。";
  String get main_title => "主标题";
  String get make_sure_service_protocol => "请确认勾选 同意服务协议与隐私条款";
  String get message_tip => "message";
  String get mine => "我的";
  String get mne_pwd_not_allow_is_null => "助记词和密码不能为空";
  String get mnemonic_backup_ok => "助记词已经记好";
  String get mnemonic_info_hint => "以下是钱包的助记词，请您务必认真抄写下来并导出至安全的地方存放。\n注意：一旦丢失，无法找回。";
  String get mnemonic_order_wrong => "验证您输入助记词不一致，建议您重新生成新钱包";
  String get mnemonic_qr_info => "这是您助记词信息生成的二维码";
  String get pls_ensure_eee_chain => "请确认勾选创建EEE链";
  String get pls_ensure_pwd_is_same => "请确认两次输入密码一致";
  String get pls_input_wallet_name => "请输入钱包名";
  String get pls_pwd_again => "请再次输入密码";
  String get pls_set_wallet_pwd => "请设置钱包密码";
  String get privacy_protocol_tag => "《隐私条款》";
  String get privacy_protocol_title => "隐私条款";
  String get public => "公告";
  String get pwd_is_not_same => "确认密码不一致，请重新输入";
  String get pwd_not_null => "密码不能为空";
  String get qr_backup => "二维码备份";
  String get qr_scan_unknown_error => "扫描发生未知失败，请重新尝试";
  String get receive => "收款";
  String get service_protocol_tag => "《服务协议》";
  String get service_protocol_title => "服务协议";
  String get some_info_is_null => "有信息为空，请补全信息";
  String get success_create_test_wallet => "测试钱包创建完成，切记注意区分钱包类型";
  String get test_wallet_and_mnemonic => "测试钱包 助记词:";
  String get test_wallet_title => "测试钱包";
  String get transfer => "转账";
  String get unknown_error_in_create_wallet => "钱包创建过程，出现位置错误，请重新尝试创建";
  String get unknown_error_in_scan_qr_code => "扫描发生未知失败，请重新尝试";
  String get verify_backup_mnemonic => "验证备份助记词";
  String get verify_mnemonic_info => "助记词确认验证";
  String get verify_mnemonic_order => "请输入验证你保存的助记词顺序。再次提醒，程序不会保留您的隐私信息,请您务必保存好助记词";
  String get verify_wallet => "验证钱包";
  String get wallet_load_error => "钱包应用加载出错了，请尝试重新打开!~";
  String get wallet_name => "钱包名";
  String get wallet_name_not_null => "钱包名不能为空";
  String get wallet_pwd => "钱包密码";
}

class $ko extends S {
  const $ko();

  @override
  TextDirection get textDirection => TextDirection.ltr;

  @override
  String get app_title => "App标题 ko";
  @override
  String get message_tip => "message ko";
  @override
  String get main_title => "主标题 ko";
}

class $jp extends S {
  const $jp();

  @override
  TextDirection get textDirection => TextDirection.ltr;

  @override
  String get app_title => "App标题 jp";
  @override
  String get message_tip => "message jp";
  @override
  String get main_title => "主标题 jp";
}

class $en extends S {
  const $en();
}

class $zh extends S {
  const $zh();

  @override
  TextDirection get textDirection => TextDirection.ltr;

  @override
  String get wallet_load_error => "钱包应用加载出错了，请尝试重新打开!~";
  @override
  String get app_title => "App标题 zh";
  @override
  String get message_tip => "message zh";
  @override
  String get main_title => "主标题 zh";
}

class GeneratedLocalizationsDelegate extends LocalizationsDelegate<S> {
  const GeneratedLocalizationsDelegate();

  List<Locale> get supportedLocales {
    return const <Locale>[
      Locale("ko", ""),
      Locale("jp", ""),
      Locale("en", ""),
      Locale("zh", ""),
    ];
  }

  LocaleListResolutionCallback listResolution({Locale fallback, bool withCountry = true}) {
    return (List<Locale> locales, Iterable<Locale> supported) {
      if (locales == null || locales.isEmpty) {
        return fallback ?? supported.first;
      } else {
        return _resolve(locales.first, fallback, supported, withCountry);
      }
    };
  }

  LocaleResolutionCallback resolution({Locale fallback, bool withCountry = true}) {
    return (Locale locale, Iterable<Locale> supported) {
      return _resolve(locale, fallback, supported, withCountry);
    };
  }

  @override
  Future<S> load(Locale locale) {
    final String lang = getLang(locale);
    if (lang != null) {
      switch (lang) {
        case "ko":
          S.current = const $ko();
          return SynchronousFuture<S>(S.current);
        case "jp":
          S.current = const $jp();
          return SynchronousFuture<S>(S.current);
        case "en":
          S.current = const $en();
          return SynchronousFuture<S>(S.current);
        case "zh":
          S.current = const $zh();
          return SynchronousFuture<S>(S.current);
        default:
          // NO-OP.
      }
    }
    S.current = const S();
    return SynchronousFuture<S>(S.current);
  }

  @override
  bool isSupported(Locale locale) => _isSupported(locale, true);

  @override
  bool shouldReload(GeneratedLocalizationsDelegate old) => false;

  ///
  /// Internal method to resolve a locale from a list of locales.
  ///
  Locale _resolve(Locale locale, Locale fallback, Iterable<Locale> supported, bool withCountry) {
    if (locale == null || !_isSupported(locale, withCountry)) {
      return fallback ?? supported.first;
    }

    final Locale languageLocale = Locale(locale.languageCode, "");
    if (supported.contains(locale)) {
      return locale;
    } else if (supported.contains(languageLocale)) {
      return languageLocale;
    } else {
      final Locale fallbackLocale = fallback ?? supported.first;
      return fallbackLocale;
    }
  }

  ///
  /// Returns true if the specified locale is supported, false otherwise.
  ///
  bool _isSupported(Locale locale, bool withCountry) {
    if (locale != null) {
      for (Locale supportedLocale in supportedLocales) {
        // Language must always match both locales.
        if (supportedLocale.languageCode != locale.languageCode) {
          continue;
        }

        // If country code matches, return this locale.
        if (supportedLocale.countryCode == locale.countryCode) {
          return true;
        }

        // If no country requirement is requested, check if this locale has no country.
        if (true != withCountry && (supportedLocale.countryCode == null || supportedLocale.countryCode.isEmpty)) {
          return true;
        }
      }
    }
    return false;
  }
}

String getLang(Locale l) => l == null
  ? null
  : l.countryCode != null && l.countryCode.isEmpty
    ? l.languageCode
    : l.toString();
