package visitor;

import java.util.ArrayList;
import java.util.List;
import visitor.user.Student;
import visitor.user.Teacher;
import visitor.user.User;
import visitor.visitor.Parent;
import visitor.visitor.Principal;
import visitor.visitor.Visitor;

/** @author Elias (siran0611@gmail.com) */
public class DataView {
  List<User> userList = new ArrayList<User>();

  public DataView() {
    userList.add(new Student("谢飞机", "重点班", "一年一班"));
    userList.add(new Student("windy", "重点班", "一年一班"));
    userList.add(new Student("大毛", "普通班", "二年三班"));
    userList.add(new Student("Shing", "普通班", "三年四班"));
    userList.add(new Teacher("BK", "特级教师", "一年一班"));
    userList.add(new Teacher("娜娜Goddess", "特级教师", "一年一班"));
    userList.add(new Teacher("dangdang", "普通教师", "二年三班"));
    userList.add(new Teacher("泽东", "实习教师", "三年四班"));
  }

  // 展示
  public void show(Visitor visitor) {
    for (User user : userList) {
      user.accept(visitor);
    }
  }

  public static void main(String[] args) {
    DataView dataView = new DataView();

    dataView.show(new Parent());
    System.out.println("==========================================");
    dataView.show(new Principal());
  }
}
