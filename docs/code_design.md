
```plantuml
@startuml
skinparam monochrome true
left to rigt direction
User1 --> (Story1)
(Story1) -> (Story2)
(Story2) --> (Story3)
Alice -> Bob: Hello
Bob -> Alice: Hi!

@enduml
```