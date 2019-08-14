import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:app/page/eee_page/chain_card.dart';
import 'package:app/page/eee_page/middle_func_card.dart';
import 'package:app/page/eee_page/digit_list_card.dart';
import '../../res/resources.dart';
import '../eee_page/left_drawer_card.dart';


class HomePage extends StatefulWidget {
  @override
  _HomePageState createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.transparent,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        brightness: Brightness.light,
        centerTitle: true,
        title: Text("这是钱包"),
      ),
      drawer: LeftDrawerCard(),
      body: Container(
          alignment: Alignment.centerLeft,
          child: Stack(
            children: <Widget>[
              new Column(
                children: <Widget>[
                  ChainCard(),
                  MiddleFuncCard(),
                  DigitListCard(),
                ],
              ),
/*              Container(
                  color: Colors.blueAccent,
                  width: ScreenUtil().setWidth(28),
                  height: ScreenUtil().setHeight(8),
                  margin: EdgeInsets.only(bottom: 5.0),
                  child: FlatButton(
                    color: Colors.white,
                    child: Text(
                      "添加以太代币",
                      style: TextStyle(color: Colors.white),
                    ),
                  ))*/
            ],
          )),
    );
  }
}