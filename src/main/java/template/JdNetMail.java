package template;

import java.util.HashMap;
import java.util.Map;

/** @author Elias (siran0611@gmail.com) */
public class JdNetMail extends NetMail {

  public JdNetMail(String uId, String uPwd) {
    super(uId, uPwd);
  }

  @Override
  protected boolean login(String uId, String uPwd) {
    System.out.println(String.format("模拟京东用户登录 uId: %s, uPwd: %s", uId, uPwd));
    return true;
  }

  @Override
  protected Map<String, String> reptile(String skuUrl) {
    Map<String, String> map = new HashMap<>();
    System.out.println(String.format("模拟京东商品信息"));
    return map;
  }

  @Override
  protected String createBase64(Map<String, String> reptile) {
    return "asasa";
  }
}
