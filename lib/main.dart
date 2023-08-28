import 'package:flutter/material.dart';

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
          children: [
            Container(
                width: 300,
                child: TextField(
                  decoration: InputDecoration(
                    labelText: '请输入连接密钥!',
                  ),
                )),
            Container(
                width: 100,
                child: TextButton(
                  child: Text('确认'),
                  onPressed: () {
                    // 确认按钮的点击事件
                    // todo 获取输入框内容
                    //
                  },
                )),
          ],
        ),
      ),
    );
  }

  //跳转聊天页面,



}
