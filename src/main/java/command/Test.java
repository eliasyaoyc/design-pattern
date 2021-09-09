package command;

/** @author Elias (siran0611@gmail.com) */
public class Test {
  public static void main(String[] args) {
    ICuisine guangDoneCuisine = new GuangDongCuisine(new GuangDongCook());
    JiangSuCuisine jiangSuCuisine = new JiangSuCuisine(new JiangSuCook());

    Waiter waiter = new Waiter();
    waiter.order(guangDoneCuisine);
    waiter.order(jiangSuCuisine);

    waiter.placeOrder();
  }
}
