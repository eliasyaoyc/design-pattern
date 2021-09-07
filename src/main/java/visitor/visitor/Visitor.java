package visitor.visitor;

import visitor.user.Student;
import visitor.user.Teacher;

/** @author Elias (siran0611@gmail.com) */
public interface Visitor {
  void visit(Student student);

  void visit(Teacher teacher);
}
