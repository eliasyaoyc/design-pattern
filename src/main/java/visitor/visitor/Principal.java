package visitor.visitor;

import visitor.user.Student;
import visitor.user.Teacher;

/** @author Elias (siran0611@gmail.com) */
public class Principal implements Visitor {

  @Override
  public void visit(Student student) {
    System.out.println(String.format("学生信息 姓名：%s 班级：%s", student.name, student.clazz));
  }

  @Override
  public void visit(Teacher teacher) {
    System.out.println(
        String.format(
            "老师信息 姓名：%s 班级：%s 升学率: %s", teacher.name, teacher.clazz, teacher.entranceRatio()));
  }
}
