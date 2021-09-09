package chain;

import java.text.ParseException;
import java.util.Date;

/** @author Elias (siran0611@gmail.com) */
public class Test {
  public static void main(String[] args) throws ParseException {
    AuthLink authLink =
        new Level3AuthLink("1000013", "王工")
            .appendNext(
                new Level2AuthLink("1000012", "张经理")
                    .appendNext(new Level1AuthLink("1000011", "段总")));

    System.out.println(
        String.format("测试结果：%s", authLink.doAuth("小傅哥", "1000998004813441", new Date())));

    // 模拟三级负责人审批
    AuthService.auth("1000013", "1000998004813441");
    System.out.println(String.format("测试结果：%s", "模拟三级负责人审批，王工"));
    System.out.println(
        String.format("测试结果：%s", authLink.doAuth("小傅哥", "1000998004813441", new Date())));

    // 模拟二级负责人审批
    AuthService.auth("1000012", "1000998004813441");
    System.out.println(String.format("测试结果：%s", "模拟二级负责人审批，张经理"));
    System.out.println(String.format("测试结果：%s", authLink.doAuth("小傅哥", "1000998004813441", new Date())));

    // 模拟一级负责人审批
    AuthService.auth("1000011", "1000998004813441");
    System.out.println(String.format("测试结果：%s", "模拟一级负责人审批，段总"));
    System.out.println(String.format("测试结果：%s", authLink.doAuth("小傅哥", "1000998004813441", new Date())));
  }
}
