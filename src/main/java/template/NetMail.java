package template;

import java.util.Map;

/** @author Elias (siran0611@gmail.com) */
public abstract class NetMail {
  String uId;
  String uPwd;

  public NetMail(String uId, String uPwd) {
    this.uId = uId;
    this.uPwd = uPwd;
  }

  public String generateGoodsPoster(String skuUrl) {
    if (!login(uId, uPwd)) return null;
    Map<String, String> reptile = reptile(skuUrl);
    return createBase64(reptile);
  }

  protected abstract boolean login(String uId, String uPwd);

  protected abstract Map<String, String> reptile(String skuUrl);

  protected abstract String createBase64(Map<String, String> reptile);

  public static void main(String[] args) {
    JdNetMail jdNetMail = new JdNetMail("yyc", "yyc");
    String aaa = jdNetMail.generateGoodsPoster("aaa");
    System.out.println(aaa);
  }
}
