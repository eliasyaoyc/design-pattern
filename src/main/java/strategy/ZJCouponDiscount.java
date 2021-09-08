package strategy;

import java.math.BigDecimal;

/** @author Elias (siran0611@gmail.com) */
public class ZJCouponDiscount implements ICouponDiscount<Double> {

  @Override
  public BigDecimal discountAmount(Double couponInfo, BigDecimal skuPrice) {
    return new BigDecimal(2);
  }
}
