package strategy;

import java.math.BigDecimal;

/** @author Elias (siran0611@gmail.com) */
public class Context<T> {
  private ICouponDiscount<T> couponDiscount;

  public Context(ICouponDiscount<T> couponDiscount) {
    this.couponDiscount = couponDiscount;
  }

  public BigDecimal discountAmount(T couponInfo, BigDecimal skuPrice) {
    return couponDiscount.discountAmount(couponInfo, skuPrice);
  }
}
