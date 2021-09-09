package command;

/** @author Elias (siran0611@gmail.com) */
public class JiangSuCuisine implements ICuisine {
  private ICook iCook;

  public JiangSuCuisine(ICook iCook) {
    this.iCook = iCook;
  }

  @Override
  public void cook() {
    iCook.doCook();
  }
}
