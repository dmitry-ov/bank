Описание/Пошаговая инструкция выполнения домашнего задания:
Реализовать библиотеку предоставляющую базовый банковский функционал:

- Создавать счёт.
- Пополнять счёт и снимать с него деньги.
- Переводить деньги со одного счёта на другой.
- Предоставлять историю операций.

Подробности:   

Счет:
- На счету хранятся деньги - целое число.
- Счёт имеет имя - уникальный идентификатор.
- Перед выполнением любых операций по счёту, его необходимо создать.
- Выполнение операции с несуществующим счётом - ошибка.
- Если счёт с именем Х существует, то создание нового счёта с именем Х - ошибка.
- Клиент может получить свой баланс.
   
Пополнение:
- Пополнение увеличивает количество денег на счете на указанную сумму.
- Пополнение на ноль денежных единиц - ошибка.
    
Снятие:
-Снятие уменьшает количество денег на счете на указанную сумму.
-Снятие нуля денежных единиц - ошибка.
-Попытка снять больше чем есть на счете - ошибка.
    
Переводы:
- Перевод уменьшает баланс отправителя и увеличивает баланс получателя на указанную сумму.
- Перевод нуля денежных единиц - ошибка.
- Перевод самому себе - ошибка.
- Если сумма перевода больше баланса отправителя - ошибка.
    
История операций:
+- Каждая операция (регистрация счёта, пополнение, снятие, перевод) должна сохраняться.
- Каждая успешная операция возвращает уникальный идентификатор, по которому данные об этой операции могут быть в дальнейшем запрошены.
- Можно получить всю историю операций.
- Можно получить историю операций связанных с конкретным счётом. Если в истории всего М операций, а со счётом X связано N операций, 
    то получение всех операций связанных со счётом Х должно выполняться за O(N), а не за О(М). 
    Иными словами, обычно М много больше N, поэтому мы должны хранить индекс операций по пользователям
- Операции должны храниться в порядке их выполнения.
- Есть возможность восстановить состояние счетов, повторно выполнив все операции из истории в новом экземпляре банка.
  
- После этого новый экземпляр банка должен совпадать с тем, историю которого мы использовали.
    Требования:
    * Библиотека реализует весь описанный функционал.
    * Весь функционал протестирован.
    * Приведён пример использования библиотеки.
    * Все публичные методы задокументированы. Документационные комментарии содержат тесты.
    * Библиотека предосталяет данные о каждой произошедшей ошибке.
    * ''cargo clippy™' n '"cargo fmt-check!' не выдают предупреждений и ошибок.

* Дополнительные задачи (по желанию):
* Счёт - обобщённый тип, реализацию которого выбирает клиент библиотеки.
* За все операции взымается комиссия и добавляется на специальный счёт.
* Банк хранит данные в базе данных (Redis, SQLite, ...).