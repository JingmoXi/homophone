import 'package:flutter/material.dart';
import 'package:homophone/chat/chatpage.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(secret: 'gdoiwgdiwhowd'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.secret});

  final String secret;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  final _textField = TextEditingController();
  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text("homophone"),
      ),
      body: Center(
        child: Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
                width: 300,
                child: TextField(
                  controller: _textField,
                  decoration: InputDecoration(
                    hintText: '请输入连接密钥!',
                    hintStyle: TextStyle(color: Colors.black, fontSize: 16),
                  ),
                )),
            Container(
                width: 100,
                child: TextButton(
                  child: Text('确认'),
                  onPressed: () {
                    // 确认按钮的点击事件
                    // todo 获取输入框内容
                    var textFieldValue = _textField.text;
                    print(textFieldValue);
                    //todo 聊天页面
                    Navigator.of(context).push(MaterialPageRoute(builder: (_) {
                      return ChatScreen(secret: textFieldValue);
                    }));

                  },
                )),
          ],
        ),
      ),
    );
  }

//跳转聊天页面,
}
