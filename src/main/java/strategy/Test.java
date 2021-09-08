package strategy;

import java.math.BigDecimal;
import java.util.HashMap;
import java.util.Map;

/** @author Elias (siran0611@gmail.com) */
public class Test {
  public static void main(String[] args) {
    Context<Map<String, String>> context = new Context<>(new MJCouponDiscount());
    BigDecimal bigDecimal = context.discountAmount(new HashMap<>(), new BigDecimal(2));
    System.out.println(bigDecimal);  // 1

    Context<Double> context1 = new Context<>(new ZJCouponDiscount());
    BigDecimal bigDecimal1 = context1.discountAmount(1D, new BigDecimal(2));
    System.out.println(bigDecimal1);  // 2
  }
}
