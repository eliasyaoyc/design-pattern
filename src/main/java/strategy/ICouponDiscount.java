package strategy;

import java.math.BigDecimal;

/** @author Elias (siran0611@gmail.com) */
public interface ICouponDiscount<T> {
  BigDecimal discountAmount(T couponInfo, BigDecimal skuPrice);
}
