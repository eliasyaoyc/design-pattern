package main

import "fmt"

type Visitor interface {
	Visit(User)
}

type Parent struct{}

func (p *Parent) Visit(user User) {
	switch f := user.(type) {
	case *Student:
		fmt.Printf("学生信息 姓名：%s 班级：%s 排名：%s\n", f.name, f.class, f.ranking())
	case *Teacher:
		fmt.Printf("老师信息 姓名：%s 班级：%s，性别：%s\n", f.name, f.class, f.identity)
	}
}

type Principle struct{}

func (p *Principle) Visit(user User) {
	switch f := user.(type) {
	case *Student:
		fmt.Printf("学生信息 姓名：%s 班级：%s\n", f.name, f.class)
	case *Teacher:
		fmt.Printf("老师信息 姓名：%s 班级：%s 升学率: %s\n", f.name, f.class, f.EntranceRatio())
	}
}

type User interface {
	Accept(Visitor)
}

type Base struct {
	name     string
	identity string
	class    string
}

type Student struct {
	*Base
}

func (s *Student) Accept(visitor Visitor) {
	visitor.Visit(s)
}

func (s *Student) ranking() uint32 {
	return 10
}

type Teacher struct {
	*Base
}

func (t *Teacher) Accept(visitor Visitor) {
	visitor.Visit(t)
}

func (t *Teacher) EntranceRatio() float32 {
	return 10.0
}

type DataView struct {
	UserList []*User
}

func main() {
}
