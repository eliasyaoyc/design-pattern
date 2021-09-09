package iterator;

/** @author Elias (siran0611@gmail.com) */
public class Link {
  private String fromId;
  private String told;

  public Link(String fromId, String told) {
    this.fromId = fromId;
    this.told = told;
  }

  public String getFromId() {
    return fromId;
  }

  public void setFromId(String fromId) {
    this.fromId = fromId;
  }

  public String getTold() {
    return told;
  }

  public void setTold(String told) {
    this.told = told;
  }
}
