#include <dqcsim>
#include <string>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim::wrap;

class MyCustomGateConverter : public CustomGateConverter {
  bool detect(Gate &&gate, QubitSet &qubits, ArbData &params) const override {
    if (!gate.is_custom() || gate.get_name() != "custom") return false;
    qubits = gate.get_targets();
    if (qubits.size() != 2) {
      throw std::runtime_error("this awesome custom gate needs two qubits");
    }
    params.push_arb_arg(42);
    return true;
  }

  Gate construct(QubitSet &&qubits, ArbData &&params) const override {
    if (qubits.size() != 2) {
      throw std::runtime_error("this awesome custom gate needs two qubits");
    }
    if (params.pop_arb_arg_as<int>() != 42) {
      throw std::runtime_error("the last arg of this custom gate needs to be 42");
    }
    auto gate = Gate::custom("custom", std::move(qubits));
    gate.set_arb(std::move(params));
    return gate;
  }
};

class MyCustomUnitaryGateConverter : public CustomUnitaryGateConverter {
  bool detect(Matrix &&matrix, ssize_t num_controls, ArbData &params) const override {
    if (!Matrix(PredefinedGate::H).approx_eq(matrix, 0.001, false)) return false;
    params.push_arb_arg((int)num_controls);
    return true;
  }

  Matrix construct(ArbData &params, ssize_t &num_controls) const override {
    num_controls = params.pop_arb_arg_as<int>();
    return Matrix(PredefinedGate::H);
  }
};

class MyBoundGate;

class MyUnboundGate {
public:
  std::string name;

  MyUnboundGate(const std::string &name) : name(name) {}

  bool operator==(const MyUnboundGate& other) const {
    return name == other.name;
  }

  MyBoundGate bind(QubitSet &&qubits, ArbData &&params) const;
};

namespace std {
  template<> struct hash<MyUnboundGate> {
    std::size_t operator()(MyUnboundGate const& gate) const noexcept {
      return std::hash<std::string>{}(gate.name);
    }
  };
}

class MyBoundGate {
public:
  std::string name;
  std::vector<QubitRef> qubits;
  ArbData params;

  MyUnboundGate get_unbound() const {
    return MyUnboundGate(name);
  }

  QubitSet get_qubits() const {
    QubitSet qubits = QubitSet();
    for (auto qubit : this->qubits) {
      qubits.push(qubit);
    }
    return qubits;
  }

  ArbData get_params() const {
    return ArbData(params);
  }
};

MyBoundGate MyUnboundGate::bind(QubitSet &&qubits, ArbData &&params) const {
  MyBoundGate gate;
  gate.name = name;
  gate.qubits = qubits.drain_into_vector();
  gate.params = std::move(params);
  return gate;
}

TEST(gatemap, test) {{
  MyUnboundGate dflt = MyUnboundGate("unknown");
  auto map = GateMap<MyUnboundGate, MyBoundGate>()
    .with_unitary(MyUnboundGate("x"), PredefinedGate::X, 0)
    .with_unitary(MyUnboundGate("cnot"), PredefinedGate::X, 1)
    .with_unitary(MyUnboundGate("z"), Matrix(PredefinedGate::Z))
    .with_unitary(MyUnboundGate("h"), std::make_shared<MyCustomUnitaryGateConverter>())
    .with_measure(MyUnboundGate("meas"))
    .with_custom(MyUnboundGate("custom"), std::make_shared<MyCustomGateConverter>());

  const MyUnboundGate *key;
  QubitSet qubits = QubitSet(0);
  ArbData params = ArbData(0);

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::predefined(
      PredefinedGate::X,
      QubitSet().with(1_q).with(2_q)
    ),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "cnot");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n        QubitRef(\n            2,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "ArbData(\n    ArbData {\n        json: Map(\n            {},\n        ),\n        args: [],\n    },\n)");

  EXPECT_ERROR(map.construct(MyUnboundGate("x"), QubitSet().with(1_q).with(2_q)), "Invalid argument: expected 0 control and 1 target qubits");
  EXPECT_EQ(map.construct(MyUnboundGate("x"), QubitSet().with(3_q), ArbData().with_arg(33)).dump(), "\
Gate(\n\
    Gate {\n\
        name: None,\n\
        targets: [\n\
            QubitRef(\n\
                3,\n\
            ),\n\
        ],\n\
        controls: [],\n\
        measures: [],\n\
        matrix: Matrix {\n\
            data: [\n\
                Complex {\n\
                    re: 0.0,\n\
                    im: 0.0,\n\
                },\n\
                Complex {\n\
                    re: 1.0,\n\
                    im: 0.0,\n\
                },\n\
                Complex {\n\
                    re: 1.0,\n\
                    im: 0.0,\n\
                },\n\
                Complex {\n\
                    re: 0.0,\n\
                    im: 0.0,\n\
                },\n\
            ],\n\
            dimension: 2,\n\
        },\n\
        data: ArbData {\n\
            json: Map(\n\
                {},\n\
            ),\n\
            args: [\n\
                [\n\
                    33,\n\
                    0,\n\
                    0,\n\
                    0,\n\
                ],\n\
            ],\n\
        },\n\
    },\n\
)");

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::predefined(
      PredefinedGate::X,
      QubitSet().with(1_q)
    ),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "x");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "ArbData(\n    ArbData {\n        json: Map(\n            {},\n        ),\n        args: [],\n    },\n)");

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::predefined(
      PredefinedGate::Y,
      QubitSet().with(1_q)
    ),
    &key, &qubits, &params
  ), false);
  EXPECT_EQ(key->name, "unknown");

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::predefined(
      PredefinedGate::Z,
      QubitSet().with(1_q)
    ),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "z");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "ArbData(\n    ArbData {\n        json: Map(\n            {},\n        ),\n        args: [],\n    },\n)");

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::predefined(
      PredefinedGate::Z,
      QubitSet().with(1_q).with(2_q)
    ),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "z");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n        QubitRef(\n            2,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "ArbData(\n    ArbData {\n        json: Map(\n            {},\n        ),\n        args: [],\n    },\n)");

  key = &dflt;
  EXPECT_EQ(map.detect(
    Gate::measure(QubitSet().with(1_q)),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "meas");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "ArbData(\n    ArbData {\n        json: Map(\n            {},\n        ),\n        args: [],\n    },\n)");

  EXPECT_EQ(
    map.construct(MyUnboundGate("meas"), QubitSet().with(3_q).with(4_q)).dump(),
    Gate::measure(QubitSet().with(3_q).with(4_q)).dump()
  );

  EXPECT_EQ(
    map.construct(MyUnboundGate("h"), QubitSet().with(3_q).with(4_q), ArbData().with_arg(1)).dump(),
    Gate::predefined(PredefinedGate::H, QubitSet().with(3_q).with(4_q)).dump()
  );

  EXPECT_ERROR(map.construct(MyUnboundGate("h"), QubitSet().with(3_q).with(4_q)), "Invalid argument: index out of range: -1");
  EXPECT_ERROR(map.construct(MyUnboundGate("h"), QubitSet().with(3_q).with(4_q), ArbData().with_arg(2)), "Invalid argument: expected 2 control and 1 target qubits");
  EXPECT_ERROR(map.construct(MyUnboundGate("h"), QubitSet().with(3_q).with(4_q), ArbData().with_arg(0)), "Invalid argument: expected 0 control and 1 target qubits");
  EXPECT_EQ(map.detect(
    Gate::predefined(PredefinedGate::H, QubitSet().with(3_q).with(4_q)),
    &key, &qubits, &params
  ), true);
  EXPECT_EQ(key->name, "h");
  EXPECT_EQ(qubits.dump(), "QubitReferenceSet(\n    [\n        QubitRef(\n            3,\n        ),\n        QubitRef(\n            4,\n        ),\n    ],\n)");
  EXPECT_EQ(params.dump(), "\
ArbData(\n\
    ArbData {\n\
        json: Map(\n\
            {},\n\
        ),\n\
        args: [\n\
            [\n\
                1,\n\
                0,\n\
                0,\n\
                0,\n\
            ],\n\
        ],\n\
    },\n\
)");

  EXPECT_ERROR(map.construct(MyUnboundGate("custom"), QubitSet().with(1_q).with(2_q)), "Invalid argument: index out of range: -1");
  EXPECT_ERROR(map.construct(MyUnboundGate("custom"), QubitSet().with(1_q).with(2_q), ArbData().with_arg(33)), "the last arg of this custom gate needs to be 42");
  EXPECT_ERROR(map.construct(MyUnboundGate("custom"), QubitSet().with(1_q), ArbData().with_arg(42)), "this awesome custom gate needs two qubits");
  Gate gate = map.construct(MyUnboundGate("custom"), QubitSet().with(3_q).with(4_q), ArbData().with_arg(33).with_arg(42));
  EXPECT_EQ(gate.dump(), "\
Gate(\n\
    Gate {\n\
        name: Some(\n\
            \"custom\",\n\
        ),\n\
        targets: [\n\
            QubitRef(\n\
                3,\n\
            ),\n\
            QubitRef(\n\
                4,\n\
            ),\n\
        ],\n\
        controls: [],\n\
        measures: [],\n\
        matrix: Matrix {\n\
            data: [],\n\
            dimension: 0,\n\
        },\n\
        data: ArbData {\n\
            json: Map(\n\
                {},\n\
            ),\n\
            args: [\n\
                [\n\
                    33,\n\
                    0,\n\
                    0,\n\
                    0,\n\
                ],\n\
            ],\n\
        },\n\
    },\n\
)");

  EXPECT_EQ(map.detect(gate, &key, &qubits, &params), true);
  EXPECT_EQ(key->name, "custom");
  EXPECT_EQ(qubits.dump(), "\
QubitReferenceSet(\n\
    [\n\
        QubitRef(\n\
            3,\n\
        ),\n\
        QubitRef(\n\
            4,\n\
        ),\n\
    ],\n\
)");
  EXPECT_EQ(params.dump(), "\
ArbData(\n\
    ArbData {\n\
        json: Map(\n\
            {},\n\
        ),\n\
        args: [\n\
            [\n\
                33,\n\
                0,\n\
                0,\n\
                0,\n\
            ],\n\
            [\n\
                42,\n\
                0,\n\
                0,\n\
                0,\n\
            ],\n\
        ],\n\
    },\n\
)");

  {
    auto result = map.convert(gate);
    EXPECT_EQ(result.name, "custom");
    EXPECT_EQ(result.qubits.size(), 2);
    EXPECT_EQ(result.qubits[0], 3_q);
    EXPECT_EQ(result.qubits[1], 4_q);
    EXPECT_EQ(params.dump(), result.params.dump());
    auto other_gate = map.convert(result);
    EXPECT_EQ(gate.dump(), other_gate.dump());
  }

  EXPECT_ERROR(map.detect(Gate::custom("custom"), &key, &qubits, &params), "this awesome custom gate needs two qubits");

  }
  dqcsim::raw::dqcs_handle_leak_check();
}
