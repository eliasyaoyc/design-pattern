package observer;

/** @author Elias (siran0611@gmail.com) */
public class MQListener implements EventListener {

  @Override
  public void onListener(Event event) {
    System.out.println(String.format("MQ 给用户 %s 发送短信 %s", event.getUser(), event.getContent()));
  }
}
