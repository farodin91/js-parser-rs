# js-parser-rs
A javascript parser in rust (WIP)


# Syntax

ReservedWord ::
  Keyword
  FutureReservedWord
  NullLiteral
  BooleanLiteral

StatementList[Yield, Return] ://done
  StatementListItem[?Yield, ?Return]...

StatementListItem[Yield, Return]://done
  Statement[?Yield, ?Return]
  Declaration[?Yield]

Declaration[Yield]://done
  HoistableDeclaration[?Yield, ~Default]
  ClassDeclaration[?Yield, ~Default]
  LexicalDeclaration[+In, ?Yield]

LexicalDeclaration[In, Yield]://done
  LetOrConst BindingList[?In, ?Yield];

LetOrConst://done
  let
  const

BindingList[In, Yield]:
  LexicalBinding[?In, ?Yield]
  BindingList[?In, ?Yield] , LexicalBinding[?In, ?Yield]

LexicalBinding[In, Yield]:
  BindingIdentifier[?Yield] Initializer[?In, ?Yield] opt
  BindingPattern[?Yield] Initializer[?In, ?Yield]


Statement [Yield, Return]:
  BlockStatement[?Yield, ?Return]
  VariableStatement[?Yield]
  EmptyStatement
  ExpressionStatement[?Yield]
  IfStatement[?Yield, ?Return]
  BreakableStatement[?Yield, ?Return]
  ContinueStatement[?Yield]
  BreakStatement[?Yield]
  [+Return]ReturnStatement[?Yield]
  WithStatement[?Yield, ?Return]
  LabelledStatement[?Yield, ?Return]
  ThrowStatement[?Yield]
  TryStatement[?Yield, ?Return]
  DebuggerStatement

EmptyStatement: ;//done


ExpressionStatement[Yield]:
  [lookahead ∉ { {, function, class, let [ }] Expression[+In, ?Yield];

Expression[In, Yield]:
  AssignmentExpression[?In, ?Yield]
  Expression[?In, ?Yield] , AssignmentExpression[?In, ?Yield]

AssignmentExpression[In, Yield]:
  ConditionalExpression[?In, ?Yield]
  [+Yield]YieldExpression[?In]
  ArrowFunction[?In, ?Yield]
  LeftHandSideExpression[?Yield] = AssignmentExpression[?In, ?Yield]
  LeftHandSideExpression[?Yield] AssignmentOperator AssignmentExpression[?In, ?Yield]

YieldExpression[In]://done
  yield
  yield[no LineTerminator here] AssignmentExpression[?In, +Yield]
  yield[no LineTerminator here] * AssignmentExpression[?In, +Yield]

ConditionalExpression: LogicalORExpression ? AssignmentExpression : AssignmentExpression

AssignmentOperator:one of
  *= /= %= += -= <<= >>= >>>= &= ^= |= **=

ArrowFunction[In, Yield]:
  ArrowParameters[?Yield] [no LineTerminator here] => ConciseBody[?In]

ArrowParameters[Yield]:
  BindingIdentifier[?Yield]
  CoverParenthesizedExpressionAndArrowParameterList[?Yield]

ConciseBody[In]:
  [lookahead ≠ {] AssignmentExpression[?In, ~Yield]
  { FunctionBody[~Yield] }

LeftHandSideExpression[Yield]:
  NewExpression[?Yield]
  CallExpression[?Yield]

NewExpression[Yield]:
  MemberExpression[?Yield]
  new NewExpression[?Yield]

MemberExpression[Yield]:
  PrimaryExpression[?Yield]
  MemberExpression[?Yield] [ Expression[+In, ?Yield] ]
  MemberExpression[?Yield] . IdentifierName
  MemberExpression[?Yield] TemplateLiteral[?Yield]
  SuperProperty[?Yield]
  MetaProperty
  new MemberExpression[?Yield] Arguments[?Yield]

PrimaryExpression[Yield]:
  this
  IdentifierReference[?Yield]
  Literal
  ArrayLiteral[?Yield]
  ObjectLiteral[?Yield]
  FunctionExpression
  ClassExpression[?Yield]
  GeneratorExpression
  RegularExpressionLiteral
  TemplateLiteral[?Yield]
  CoverParenthesizedExpressionAndArrowParameterList[?Yield]

CoverParenthesizedExpressionAndArrowParameterList[Yield]:
  ( Expression[+In, ?Yield] )
  ( )
  ( ...BindingIdentifier[?Yield] )
  ( ...BindingPattern[?Yield] )
  ( Expression[+In, ?Yield] , ... BindingIdentifier[?Yield] )
  ( Expression[+In, ?Yield] , ... BindingPattern[?Yield] )



BlockStatement[Yield, Return]://done
  Block[?Yield, ?Return]

Block[Yield, Return]://done
  { StatementList[?Yield, ?Return]opt }

IfStatement[Yield, Return]://done
  if ( Expression[+In, ?Yield] ) Statement[?Yield, ?Return] else Statement[?Yield, ?Return]
  if ( Expression[+In, ?Yield] ) Statement[?Yield, ?Return]

BreakableStatement[Yield, Return]://done
  IterationStatement[?Yield, ?Return]
  SwitchStatement[?Yield, ?Return]

IterationStatement[Yield, Return]:
  do Statement[?Yield, ?Return] while ( Expression[+In, ?Yield] ) ;
  while ( Expression[+In, ?Yield] ) Statement[?Yield, ?Return]
  for ( [lookahead ∉ { let [ }] Expression[~In, ?Yield] opt ; Expression[+In, ?Yield] opt ; Expression[+In, ?Yield] opt ) Statement[?Yield, ?Return]
  for ( var VariableDeclarationList[~In, ?Yield] ; Expression[+In, ?Yield] opt ; Expression[+In, ?Yield] opt ) Statement[?Yield, ?Return]
  for ( LexicalDeclaration[~In, ?Yield] Expression[+In, ?Yield] opt ; Expression[+In, ?Yield] opt ) Statement[?Yield, ?Return]
  for ( [lookahead ∉ { let [ }]LeftHandSideExpression[?Yield] in Expression[+In, ?Yield] ) Statement[?Yield, ?Return]
  for ( var ForBinding[?Yield] in Expression[+In, ?Yield])Statement[?Yield, ?Return]
  for ( ForDeclaration[?Yield] in Expression[+In, ?Yield])Statement[?Yield, ?Return]
  for ( [lookahead ≠ let]LeftHandSideExpression[?Yield] of AssignmentExpression[+In, ?Yield] ) Statement[?Yield, ?Return]
  for ( var ForBinding[?Yield] of AssignmentExpression[+In, ?Yield] ) Statement[?Yield, ?Return]
  for ( ForDeclaration[?Yield] of AssignmentExpression[+In, ?Yield] ) Statement[?Yield, ?Return]


SwitchStatement[Yield, Return]://done
  switch ( Expression[+In, ?Yield] ) CaseBlock[?Yield, ?Return]

CaseBlock[Yield, Return]://done
  { CaseClauses[?Yield, ?Return] opt }
  { CaseClauses[?Yield, ?Return] opt DefaultClause[?Yield, ?Return] CaseClauses[?Yield, ?Return] opt }

CaseClauses[Yield, Return]://done
  CaseClause[?Yield, ?Return]
  CaseClauses[?Yield, ?Return] CaseClause[?Yield, ?Return]

CaseClause[Yield, Return]://done
  case Expression[+In, ?Yield] : StatementList[?Yield, ?Return] opt

ContinueStatement[Yield]://done
  continue ;
  continue [no LineTerminator here] LabelIdentifier[?Yield] ;

BreakStatement[Yield]:
  break ;
  break [no LineTerminator here] LabelIdentifier[?Yield] ;

WithStatement[Yield, Return]://done
  with ( Expression[+In, ?Yield] ) Statement[?Yield, ?Return]

LabelledStatement[Yield, Return]://done
  LabelIdentifier[?Yield] : LabelledItem[?Yield, ?Return]

LabelledItem[Yield, Return]://done
  Statement[?Yield, ?Return]
  FunctionDeclaration[?Yield, ~Default]

ThrowStatement[Yield]://done
  throw [no LineTerminator here] Expression[+In, ?Yield] ;

TryStatement[Yield, Return]://done
  try Block[?Yield, ?Return] Catch[?Yield, ?Return]
  try Block[?Yield, ?Return] Finally[?Yield, ?Return]
  try Block[?Yield, ?Return] Catch[?Yield, ?Return] Finally[?Yield, ?Return]

Catch[Yield, Return]://done
  catch ( CatchParameter[?Yield] ) Block[?Yield, ?Return]

Finally[Yield, Return]://done
  finally Block[?Yield, ?Return]

CatchParameter[Yield]:
  BindingIdentifier[?Yield]
  BindingPattern[?Yield]

DebuggerStatement: debugger ;  //done

DefaultClause[Yield, Return]:
  default : StatementList[?Yield, ?Return] opt

ForDeclaration[Yield]:
  LetOrConst ForBinding[?Yield]

ForBinding[Yield]:
  BindingIdentifier[?Yield]
  BindingPattern[?Yield]

BindingPattern[Yield]:
  ObjectBindingPattern[?Yield]
  ArrayBindingPattern[?Yield]


VariableStatement[Yield]://done
  var VariableDeclarationList[+In, ?Yield];

VariableDeclarationList[In, Yield]:
  VariableDeclaration[?In, ?Yield]
  VariableDeclarationList[?In, ?Yield],VariableDeclaration[?In, ?Yield]

VariableDeclaration[In, Yield]:
  BindingIdentifier[?Yield] Initializer[?In, ?Yield] opt
  BindingPattern[?Yield] Initializer[?In, ?Yield]

BindingIdentifier[Yield]:
  Identifier
  [~Yield]yield

LabelIdentifier[Yield]:
  Identifier
  [~Yield]yield

Initializer[In, Yield]://done
  = AssignmentExpression[?In, ?Yield]


IdentifierReference[Yield]:
  Identifier
  [~Yield]yield

Identifier:
  IdentifierName but not ReservedWord
