package command;

import java.util.ArrayList;
import java.util.List;

/** @author Elias (siran0611@gmail.com) */
public class Waiter {
  private List<ICuisine> cuisines = new ArrayList<>();

  public void order(ICuisine iCuisine) {
    cuisines.add(iCuisine);
  }

  public synchronized void placeOrder() {
    cuisines.forEach(ICuisine::cook);
  }
}
