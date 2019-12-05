#include <dqcsim>
#include <memory>
#include "util.hpp"
#include "gtest/gtest.h"

using namespace dqcsim;

TEST(gate, controlled) {
  std::shared_ptr<wrap::Gate> gate;
  {
    wrap::QubitSet control;
    control.push(wrap::QubitRef(1));
    wrap::QubitSet target;
    target.push(wrap::QubitRef(3));
    wrap::Matrix matrix(2);
    gate = std::make_shared<wrap::Gate>(wrap::Gate::unitary(target, control, matrix));
  }
  EXPECT_EQ(gate->get_targets().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            3,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_targets());
  EXPECT_EQ(gate->get_controls().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_controls());
  EXPECT_EQ(gate->get_measures().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_measures());
  EXPECT_EQ(gate->get_matrix().size(), 2);
  EXPECT_TRUE(gate->has_matrix());
  EXPECT_ERROR(gate->get_name(), "Invalid argument: gate is not custom and thus does not have a name");
  EXPECT_FALSE(gate->is_custom());
}

TEST(gate, unitary) {
  std::shared_ptr<wrap::Gate> gate;
  {
    wrap::QubitSet target;
    target.push(wrap::QubitRef(2));
    target.push(wrap::QubitRef(3));
    wrap::Matrix matrix(4);
    gate = std::make_shared<wrap::Gate>(wrap::Gate::unitary(target, matrix));
  }
  EXPECT_EQ(gate->get_targets().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            2,\n        ),\n        QubitRef(\n            3,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_targets());
  EXPECT_EQ(gate->get_controls().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_controls());
  EXPECT_EQ(gate->get_measures().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_measures());
  EXPECT_EQ(gate->get_matrix().size(), 4);
  EXPECT_TRUE(gate->has_matrix());
  EXPECT_ERROR(gate->get_name(), "Invalid argument: gate is not custom and thus does not have a name");
  EXPECT_FALSE(gate->is_custom());
}

TEST(gate, measurement) {
  std::shared_ptr<wrap::Gate> gate;
  {
    wrap::QubitSet measures;
    measures.push(wrap::QubitRef(4));
    gate = std::make_shared<wrap::Gate>(wrap::Gate::measure(measures));
  }
  EXPECT_EQ(gate->get_targets().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_targets());
  EXPECT_EQ(gate->get_controls().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_controls());
  EXPECT_EQ(gate->get_measures().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            4,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_measures());
  EXPECT_ERROR(gate->get_matrix().size(), "Invalid argument: no matrix associated with gate");
  EXPECT_FALSE(gate->has_matrix());
  EXPECT_ERROR(gate->get_name(), "Invalid argument: gate is not custom and thus does not have a name");
  EXPECT_FALSE(gate->is_custom());
}

TEST(gate, custom_matrix) {
  std::shared_ptr<wrap::Gate> gate;
  {
    wrap::QubitSet control;
    control.push(wrap::QubitRef(1));
    wrap::QubitSet target;
    target.push(wrap::QubitRef(2));
    target.push(wrap::QubitRef(3));
    wrap::QubitSet measures;
    measures.push(wrap::QubitRef(4));
    wrap::Matrix matrix(4);
    gate = std::make_shared<wrap::Gate>(wrap::Gate::custom("test", target, control, measures, matrix));
  }
  EXPECT_EQ(gate->get_targets().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            2,\n        ),\n        QubitRef(\n            3,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_targets());
  EXPECT_EQ(gate->get_controls().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            1,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_controls());
  EXPECT_EQ(gate->get_measures().dump(), std::string("QubitReferenceSet(\n    [\n        QubitRef(\n            4,\n        ),\n    ],\n)"));
  EXPECT_TRUE(gate->has_measures());
  EXPECT_EQ(gate->get_matrix().size(), 4);
  EXPECT_TRUE(gate->has_matrix());
  EXPECT_EQ(gate->get_name(), "test");
  EXPECT_TRUE(gate->is_custom());
}

TEST(gate, custom_no_matrix) {
  std::shared_ptr<wrap::Gate> gate;
  {
    wrap::QubitSet empty;
    gate = std::make_shared<wrap::Gate>(wrap::Gate::custom("test", empty, empty, empty));
  }
  EXPECT_EQ(gate->get_targets().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_targets());
  EXPECT_EQ(gate->get_controls().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_controls());
  EXPECT_EQ(gate->get_measures().dump(), std::string("QubitReferenceSet(\n    [],\n)"));
  EXPECT_FALSE(gate->has_measures());
  EXPECT_ERROR(gate->get_matrix().size(), "Invalid argument: no matrix associated with gate");
  EXPECT_FALSE(gate->has_matrix());
  EXPECT_EQ(gate->get_name(), "test");
  EXPECT_TRUE(gate->is_custom());
}
