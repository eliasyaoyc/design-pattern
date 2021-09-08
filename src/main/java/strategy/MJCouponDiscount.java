package strategy;

import java.math.BigDecimal;
import java.util.Map;

/** @author Elias (siran0611@gmail.com) */
public class MJCouponDiscount implements ICouponDiscount<Map<String, String>> {

  @Override
  public BigDecimal discountAmount(Map<String, String> couponInfo, BigDecimal skuPrice) {
    return new BigDecimal(1);
  }
}
