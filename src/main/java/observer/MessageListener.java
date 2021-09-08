package observer;

/** @author Elias (siran0611@gmail.com) */
public class MessageListener implements EventListener {

  @Override
  public void onListener(Event event) {
    System.out.println(
        String.format("MESSAGE 给用户 %s 发送短信 %s", event.getUser(), event.getContent()));
  }
}
