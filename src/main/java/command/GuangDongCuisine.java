package command;

/** @author Elias (siran0611@gmail.com) */
public class GuangDongCuisine implements ICuisine {
  private ICook iCook;

  public GuangDongCuisine(ICook iCook) {
    this.iCook = iCook;
  }

  @Override
  public void cook() {
    iCook.doCook();
  }
}
