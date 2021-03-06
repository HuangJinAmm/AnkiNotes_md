### DI（Dependency Injection，依赖注入）
---
- 当某个角色 需要另外一个角色协助的时候，在传统的程序设计过程中，通常由调用者来创建被调用者的实例。但在spring中 创建被调用者的工作不再由调用者来完成，因此称为控制反转。创建被调用者的工作由spring来完成，然后注入调用者 因此也称为依赖注入。 
- 在Spring创建对象的过程中,把对象依赖的属性注入到类中。 
- ●谁依赖于谁：当然是应用程序依赖内部的属性 
  ●为什么需要依赖：应用程序需要IoC容器来提供对象需要的外部资源； 
  ●谁注入谁：很明显是IoC容器注入应用程序某个对象，应用程序依赖的对象； 
  ●注入了什么：就是注入某个对象所需要的外部资源（包括对象、资源、常量数据）。


### IOC（inversion of control，控制反转）
---
- 是通过IOC容器实现的，由IOC容器负责创建和获取依赖对象，对象只是被动地接受依赖对象。
- IOC，另外一种说法叫DI（Dependency Injection），即依赖注入。它是一种设计思想。在任何一个有实际开发意义的程序项目中，我们会使用很多类来描述它们特有的功能，并且通过类与类之间的相互协作来完成特定的业务逻辑。这个时候，每个类都需要负责管理与自己有交互的类的引用和依赖，代码将会变的异常难以维护和极度的高耦合。而IOC的出现正是用来解决这个问题，我们通过IOC将这些相互依赖对象的创建、协调工作交给Spring容器去处理，每个对象只需要关注其自身的业务逻辑关系就可以了。在这样的角度上来看，获得依赖的对象的方式，进行了反转，变成了由spring容器控制对象如何获取外部资源（包括其他对象和文件资料等等）。
- 例子

- 面向对象设计法则之一—— 好莱坞法则：“别找我们，我们找你”；即由IoC容器帮对象找相应的依赖对象并注入，而不是由对象主动去找。
- 控制指的是：当前对象对内部成员的控制权。
  反转指的是：这种控制权不由当前对象管理了，由其他(类,第三方容器)来管理。获得依赖对象的方式反转了。

- 原理就是通过Java的反射技术来实现的！通过反射我们可以获取类的所有信息(成员变量、类名等等等)！
  再通过配置文件(xml)或者注解来描述类与类之间的关系
  我们就可以通过这些配置信息和反射技术来构建出对应的对象和依赖关系了！


### AOP（Aspect Oriented Programming，面向切面编程）
---
- AOP的本质是在一系列纵向的控制流程中，把那些相同的子流程提取成一个横向的面

### 简化java开发

- 激发pojo的潜能
- 依赖注入
- 应用切面
- 使用模版消除样板式代码


### 容纳你的bean
---
- 与应用上下文共事
- bean的生命周期

