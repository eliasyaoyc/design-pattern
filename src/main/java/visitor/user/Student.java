package visitor.user;

import visitor.visitor.Visitor;

/** @author Elias (siran0611@gmail.com) */
public class Student extends User {
  public Student(String name, String identity, String clazz) {
    super(name, identity, clazz);
  }

  @Override
  public void accept(Visitor visitor) {
    visitor.visit(this);
  }

  public int ranking() {
    return (int) (Math.random() * 100);
  }
}
