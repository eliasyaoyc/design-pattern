package visitor.user;

import java.math.BigDecimal;
import visitor.visitor.Visitor;

/** @author Elias (siran0611@gmail.com) */
public class Teacher extends User {

  public Teacher(String name, String identity, String clazz) {
    super(name, identity, clazz);
  }

  @Override
  public void accept(Visitor visitor) {
    visitor.visit(this);
  }

  public double entranceRatio() {
    return BigDecimal.valueOf(Math.random() * 100)
        .setScale(2, BigDecimal.ROUND_HALF_UP)
        .doubleValue();
  }
}
