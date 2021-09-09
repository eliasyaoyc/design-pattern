package chain;

import java.util.Date;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/** @author Elias (siran0611@gmail.com) */
public class AuthService {
  private static Map<String, Date> authMap = new ConcurrentHashMap<>();

  public static Date queryAuthInfo(String uId, String orderId) {
    return authMap.get(uId.concat(orderId));
  }

  public static void auth(String uId, String orderId) {
    authMap.put(uId.concat(orderId), new Date());
  }
}
