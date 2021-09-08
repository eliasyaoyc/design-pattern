package observer;

import observer.EventManager.ListenerType;

/** @author Elias (siran0611@gmail.com) */
public abstract class BusinessService {
  private EventManager eventManager;

  public BusinessService() {
    eventManager = new EventManager(ListenerType.MQ, ListenerType.MESSAGE);
    eventManager.subscribe(ListenerType.MQ, new MQListener());
    eventManager.subscribe(ListenerType.MESSAGE, new MessageListener());
  }

  public void handle(String user, String content) {
    Event event = doHandle(user, content);
    eventManager.notify(ListenerType.MQ, event);
    eventManager.notify(ListenerType.MESSAGE, event);
  }

  protected abstract Event doHandle(String user, String content);

  public static class Impl extends BusinessService {

    @Override
    protected Event doHandle(String user, String content) {
      return new Event(user,content);
    }
  }
}
