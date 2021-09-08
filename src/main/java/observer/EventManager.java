package observer;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

/** @author Elias (siran0611@gmail.com) */
public class EventManager {
  private Map<Enum<ListenerType>, List<EventListener>> listeners = new HashMap<>();

  public EventManager(Enum<ListenerType>... operations) {
    for (Enum<ListenerType> operation : operations) {
      listeners.put(operation, new ArrayList<>());
    }
  }

  public enum ListenerType {
    MQ,
    MESSAGE
  }

  public void subscribe(Enum<ListenerType> listenerType, EventListener listener) {
    listeners.get(listenerType).add(listener);
  }

  public void unsubscribe(Enum<ListenerType> listenerType, EventListener listener) {
    listeners.get(listenerType).remove(listener);
  }

  public void notify(Enum<ListenerType> listenerType, Event event) {
    listeners.get(listenerType).forEach(listener -> listener.onListener(event));
  }
}
