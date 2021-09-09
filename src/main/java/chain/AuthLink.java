package chain;

import java.text.SimpleDateFormat;
import java.util.Date;

/** @author Elias (siran0611@gmail.com) */
public abstract class AuthLink {
  protected SimpleDateFormat f = new SimpleDateFormat("yyyy-MM-dd HH:mm:ss");
  protected String levelUserId;
  protected String levelUserName;
  private AuthLink next;

  public AuthLink(String levelUserId, String levelUserName) {
    this.levelUserId = levelUserId;
    this.levelUserName = levelUserName;
  }

  public AuthLink next() {
    return next;
  }

  public AuthLink appendNext(AuthLink next) {
    this.next = next;
    return this;
  }

  public abstract AuthInfo doAuth(String uId, String orderId, Date authDate);
}
