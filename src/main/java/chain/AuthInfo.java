package chain;

/** @author Elias (siran0611@gmail.com) */
public class AuthInfo {
  private String code;
  private String info = "";

  public AuthInfo(String code, String info) {
    this.code = code;
    this.info = info;
  }

  public String getCode() {
    return code;
  }

  public void setCode(String code) {
    this.code = code;
  }

  public String getInfo() {
    return info;
  }

  public void setInfo(String info) {
    this.info = info;
  }
}
