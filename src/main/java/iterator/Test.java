package iterator;

/** @author Elias (siran0611@gmail.com) */
public class Test {
  public static void main(String[] args) {
    // 数据填充
    GroupStructure groupStructure = new GroupStructure("1", "小傅哥");

    // 雇员信息
    groupStructure.add(new Employee("2", "花花", "二级部门"));
    groupStructure.add(new Employee("3", "豆包", "二级部门"));
    groupStructure.add(new Employee("4", "蹦蹦", "三级部门"));
    groupStructure.add(new Employee("5", "大烧", "三级部门"));
    groupStructure.add(new Employee("6", "虎哥", "四级部门"));
    groupStructure.add(new Employee("7", "玲姐", "四级部门"));
    groupStructure.add(new Employee("8", "秋雅", "四级部门"));

    // 节点关系 1->(1,2) 2->(4,5)
    groupStructure.addLink("1", new Link("1", "2"));
    groupStructure.addLink("1", new Link("1", "3"));
    groupStructure.addLink("2", new Link("2", "4"));
    groupStructure.addLink("2", new Link("2", "5"));
    groupStructure.addLink("5", new Link("5", "6"));
    groupStructure.addLink("5", new Link("5", "7"));
    groupStructure.addLink("5", new Link("5", "8"));

    Iterator<Employee> iterator = groupStructure.iterator();
    while (iterator.hasNext()) {
      Employee employee = iterator.next();
      System.out.println(String.format("%s，雇员 Id：%s Name：%s", employee.getDesc(), employee.getId(), employee.getName()));
    }
  }
}
