import 'package:app/model/digit.dart';

class Rate {
  static Rate get instance => _getInstance();
  static Rate _instance; //private

  static _getInstance() {
    if (_instance == null) {
      _instance = new Rate._internal();
      return _instance;
    }
    return _instance;
  }

  Rate._internal() {
    // 可初始化参数
  }

  Map digitRateMap = Map<String, DigitRate>();
  Map legalMap = Map<String, double>();
  String nowLegalCurrent = "USD";

  setDigitRateMap(Map digitRateMap) {
    this.digitRateMap = digitRateMap;
  }

  setLegalMap(Map map) {
    this.legalMap = map;
  }

  List<String> getAllSupportLegalCurrency() {
    return List.from(this.legalMap.keys);
  }

  String setNowLegalCurrency() {
    return this.nowLegalCurrent;
  }

  double getNowLegalCurrency(String legal) {
    return this.legalMap[legal];
  }

  double getMoney(Digit digit) {
    return getPrice(digit) * double.parse(digit.balance);
  }

  double getChangeHourly(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return 0.0;
    }
    return this.digitRateMap[digit.shortName.toUpperCase()]["changeHourly"];
  }

  String getName(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return "";
    }
    return this.digitRateMap[digit.shortName.toUpperCase()]["name"];
  }

  String getSymbol(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return "";
    }
    return this.digitRateMap[digit.shortName.toUpperCase()]["symbol"];
  }

  double getPrice(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return 0.0;
    }
    return instance.digitRateMap[digit.shortName.toUpperCase()]["price"] * instance.legalMap[nowLegalCurrent];
  }

  double getHigh(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return 0.0;
    }
    return instance.digitRateMap[digit.shortName.toUpperCase()]["high"];
  }

  double getLow(Digit digit) {
    if (!digitRateMap.containsKey(digit.shortName.toUpperCase())) {
      return 0.0;
    }
    return instance.digitRateMap[digit.shortName.toUpperCase()]["low"];
  }
}

//api返回model格式一致
class DigitRate {
  String name = "";
  String symbol = "";
  double price = 0.0;
  double high = 0.0;
  double low = 0.0;
  double histHigh = 0.0;
  double histLow = 0.0;
  int timestamps = 0;
  int volume = 0;
  double changeHourly = 0.00;

  String get getChangeHour {
    if (changeHourly > 0) {
      return (changeHourly * 100.0).toString() + "%" + "↑";
    }
    return (changeHourly * 100.0).toString() + "%" + "↓";
  }
}
