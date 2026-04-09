import { invoke } from "@tauri-apps/api/core";
import { toList } from "./build/dev/javascript/shift_manager_frontend/gleam.mjs";

export function listPlans() {
  return invoke("list_all_plans").then((plans) =>
    toList(plans.map((plan) => [plan.id, plan.name])),
  );
}

export function currentDate() {
  const now = new Date();
  return [now.getFullYear(), now.getMonth()];
}

export function calendarGrid(year, month) {
  const firstDay = new Date(year, month, 1);
  const dayOfWeek = (firstDay.getDay() + 6) % 7;
  const startDate = new Date(firstDay);
  startDate.setDate(firstDay.getDate() - dayOfWeek);

  const current = new Date(startDate);
  const weeks = [];

  for (let week = 0; week < 6; week += 1) {
    const days = [];
    let hasCurrentMonthDay = false;

    for (let day = 0; day < 7; day += 1) {
      const date = new Date(current);
      const inCurrentMonth = date.getMonth() === month;
      if (inCurrentMonth) {
        hasCurrentMonthDay = true;
      }
      days.push([date.getDate(), inCurrentMonth]);
      current.setDate(current.getDate() + 1);
    }

    if (!hasCurrentMonthDay && weeks.length > 0) {
      break;
    }

    const monday = new Date(daysToDate(startDate, week * 7));
    const weekKey = monday.toISOString().split("T")[0];
    weeks.push([weekKey, toList(days)]);
  }

  return toList(weeks);
}

function daysToDate(startDate, offset) {
  const date = new Date(startDate);
  date.setDate(startDate.getDate() + offset);
  return date;
}

export function promptText(title, defaultValue) {
  return Promise.resolve(globalThis.prompt(title, defaultValue) ?? "");
}

export function createPlan(name) {
  return invoke("create_new_plan", { name });
}

export function getPlanConfig(planId) {
  return invoke("get_plan_config", { planId }).then((config) => [
    [config.plan.id, config.plan.name],
    toList(
      config.groups.map((entry) => [
        entry.group.id,
        entry.group.name,
        toList(
          entry.members.map((member) => [member.id, member.name, member.sort_order]),
        ),
      ]),
    ),
    toList(
      config.rules.map((entry) => [
        entry.rule.id,
        entry.rule.name,
        toList(
          entry.assignments.map((assignment) => [
            assignment.id,
            normalizeWeekday(assignment.weekday),
            normalizeShiftTime(assignment.shift_time_type),
            assignment.target_group_id,
            assignment.target_member_index,
          ]),
        ),
      ]),
    ),
  ]);
}

export function addStaffGroup(planId, name) {
  return invoke("add_staff_group", { planId, name });
}

export function deleteStaffGroup(groupId) {
  return invoke("delete_staff_group", { groupId });
}

export function updateGroupName(groupId, name) {
  return invoke("update_group_name", { groupId, name });
}

export function addStaffMember(groupId, name) {
  return invoke("add_staff_member", { groupId, name });
}

export function deleteStaffMember(memberId) {
  return invoke("delete_staff_member", { memberId });
}

export function updateMemberName(memberId, name) {
  return invoke("update_member_name", { memberId, name });
}

export function addWeeklyRule(planId, name) {
  return invoke("add_weekly_rule", { planId, name });
}

export function deleteWeeklyRule(ruleId) {
  return invoke("delete_weekly_rule", { ruleId });
}

export function updateRuleName(ruleId, name) {
  return invoke("update_rule_name", { ruleId, name });
}

export function addAssignment(ruleId, weekday, shiftTime, groupId, memberIndex) {
  return invoke("add_rule_assignment", {
    ruleId,
    weekday,
    shiftTime,
    groupId,
    memberIndex,
  });
}

export function deleteAssignment(assignmentId) {
  return invoke("delete_assignment", { assignmentId });
}

export function getCalendarState(planId) {
  return invoke("get_calendar_state", { planId }).then((state) =>
    state ? [true, state.initialDelta, state.baseAbsWeek] : [false, 0, 0],
  );
}

export function updateInitialDelta(planId, initialDelta) {
  return invoke("update_initial_delta", { planId, initialDelta });
}

export function deleteFutureShifts(planId, year, month) {
  return invoke("delete_future_shifts", { planId, year, month });
}

export function generateAndSaveShift(
  planId,
  skips,
  year,
  month,
  useInitialDelta,
  initialDelta,
) {
  return invoke("generate_and_save_shift", {
    planId,
    skips: skips.toArray(),
    year,
    month,
    initialDelta: useInitialDelta ? initialDelta : undefined,
  });
}

export function deriveMonthlyShift(planId, targetYear, targetMonth) {
  return invoke("derive_monthly_shift", { planId, targetYear, targetMonth }).then(
    (result) =>
      toList(
        result.weeks.map((week) => [
          week.status,
          toList(
            (week.shift?.days ?? []).map((day) => [
              toList(day.morning ?? []),
              toList(day.afternoon ?? []),
            ]),
          ),
        ]),
      ),
  );
}

function normalizeWeekday(value) {
  if (typeof value === "number") {
    return value;
  }

  switch (value) {
    case "Monday":
      return 0;
    case "Tuesday":
      return 1;
    case "Wednesday":
      return 2;
    case "Thursday":
      return 3;
    case "Friday":
      return 4;
    case "Saturday":
      return 5;
    case "Sunday":
      return 6;
    default:
      return -1;
  }
}

function normalizeShiftTime(value) {
  if (typeof value === "number") {
    return value;
  }

  switch (value) {
    case "Morning":
      return 0;
    case "Afternoon":
      return 1;
    default:
      return -1;
  }
}
