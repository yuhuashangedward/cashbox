import 'package:app/demo/flutter_webview_plugin_demo.dart';
import 'package:app/demo/local_html_webview_demo.dart';
import 'package:app/page/digit_list_page/digit_list_page.dart';
import 'package:app/page/sign_tx_page/sign_tx_page.dart';
import 'package:flutter/material.dart';
import 'package:fluro/fluro.dart';

import '../page/entry_page.dart';
import 'package:app/page/eth_page/eth_page.dart';
import '../page/public_page/public_page.dart';
import '../page/create_wallet_page/create_wallet_name_page.dart';
import '../page/create_wallet_page/create_wallet_mnemonic_page.dart';
import '../page/create_wallet_page/create_wallet_confirm_page.dart';
import '../page/qr_info_page/qr_info_page.dart';
import '../page/import_wallet_page/import_wallet_page.dart';
import '../page/transfer_eth_page/transfer_eth_page.dart';
import '../page/mine_page/mine_page.dart';
import '../page/wallet_manager_list_page/wallet_manager_list_page.dart';
import '../page/wallet_manager_page/wallet_manage_page.dart';
import '../page/reset_pwd_page/reset_pwd_page.dart';
import '../page/recover_wallet_page/recover_wallet_page.dart';
import '../page/transaction_history_page/transaction_history_page.dart';
import 'package:app/page/user_protocol/privacy_statements_zh.dart';
import 'package:app/page/user_protocol/service_agreement_zh.dart';
import 'package:app/page/transaction_detail_page/eth_transactin_detail_page.dart';
import '../page/about_us_page/about_us_page.dart';
import 'package:app/demo/dapp_webview_demo.dart';
import 'package:app/page/dapp_page/dapp_page.dart';
import 'package:app/page/create_test_wallet_page/create_test_wallet_page.dart';

Handler entryPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return EntryPage();
});

Handler ethPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  bool isForceLoadFromJni = params['isForceLoadFromJni'].first == 'true';
  return EthPage(isForceLoadFromJni: isForceLoadFromJni);
});

Handler publicPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return PublicPage();
});

Handler dappPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return DappPage();
});

Handler dappDemoPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return DAppWebViewDemo();
});

Handler createWalletPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return CreateWalletNamePage();
});

Handler digitListPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return DigitListPage();
});

Handler createWalletMnemonicPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return CreateWalletMnemonicPage();
});

Handler createWalletConfirmPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return CreateWalletConfirmPage();
});

Handler qrInfoPageHandler = Handler(handlerFunc: (context, params) {
  return QrInfoPage();
});

Handler importWalletPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return ImportWalletPage();
});

Handler transferEthPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return TransferEthPage();
});

Handler minePageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return MinePage();
});

Handler walletManagerListPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return WalletManagerListPage();
});

Handler walletManagerPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return WalletManagerPage();
});

Handler resetPwdPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return ResetPwdPage();
});

Handler recoverWalletPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return RecoverWalletPage();
});

Handler privacyStatementHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return PrivacyStatementPage();
});

Handler serviceAgreementHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return ServiceAgreementPage();
});

Handler transactionHistoryHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return TransactionHistoryPage();
});

Handler transactionEeeDetailHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return EeeTransactionDetailPage();
});

Handler aboutUsPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return AboutUsPage();
});

Handler createTestWalletHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return CreateTestWalletPage();
});
Handler signTxPageHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return SignTxPage();
});

Handler flutterWebViewPluginHandler = Handler(handlerFunc: (BuildContext context, Map<String, List<String>> params) {
  return FlutterWebViewPluginDemo();
});
