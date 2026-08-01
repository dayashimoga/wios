import 'package:equatable/equatable.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:wios_common/wios_common.dart';
import 'package:wios_domain/wios_domain.dart';

// ── Events ───────────────────────────────────────────────────────

abstract class SystemEvent extends Equatable {
  const SystemEvent();
  @override
  List<Object?> get props => [];
}

class SystemInitRequested extends SystemEvent {
  const SystemInitRequested();
}

class SystemRefreshRequested extends SystemEvent {
  const SystemRefreshRequested();
}

// ── State ────────────────────────────────────────────────────────

enum SystemStatus { initial, loading, ready, error }

class SystemState extends Equatable {
  final SystemStatus status;
  final AppInfo? appInfo;
  final DeviceCapabilities? capabilities;
  final String? config;
  final String? errorMessage;

  const SystemState({
    this.status = SystemStatus.initial,
    this.appInfo,
    this.capabilities,
    this.config,
    this.errorMessage,
  });

  SystemState copyWith({
    SystemStatus? status,
    AppInfo? appInfo,
    DeviceCapabilities? capabilities,
    String? config,
    String? errorMessage,
  }) =>
      SystemState(
        status: status ?? this.status,
        appInfo: appInfo ?? this.appInfo,
        capabilities: capabilities ?? this.capabilities,
        config: config ?? this.config,
        errorMessage: errorMessage ?? this.errorMessage,
      );

  @override
  List<Object?> get props =>
      [status, appInfo, capabilities, config, errorMessage];
}

// ── BLoC ─────────────────────────────────────────────────────────

class SystemBloc extends Bloc<SystemEvent, SystemState> {
  final NodeRepository _nodeRepo;

  SystemBloc({required NodeRepository nodeRepository})
      : _nodeRepo = nodeRepository,
        super(const SystemState()) {
    on<SystemInitRequested>(_onInit);
    on<SystemRefreshRequested>(_onRefresh);
  }

  Future<void> _onInit(
    SystemInitRequested event,
    Emitter<SystemState> emit,
  ) async {
    emit(state.copyWith(status: SystemStatus.loading));
    try {
      await _nodeRepo.initialize();
      final appInfo = await _nodeRepo.getAppInfo();
      final caps = await _nodeRepo.getDeviceCapabilities();
      final config = await _nodeRepo.getDefaultConfig();
      emit(state.copyWith(
        status: SystemStatus.ready,
        appInfo: appInfo,
        capabilities: caps,
        config: config,
      ));
    } catch (e) {
      emit(state.copyWith(
        status: SystemStatus.error,
        errorMessage: e.toString(),
      ));
    }
  }

  Future<void> _onRefresh(
    SystemRefreshRequested event,
    Emitter<SystemState> emit,
  ) async {
    try {
      final appInfo = await _nodeRepo.getAppInfo();
      final caps = await _nodeRepo.getDeviceCapabilities();
      emit(state.copyWith(appInfo: appInfo, capabilities: caps));
    } catch (e) {
      emit(state.copyWith(errorMessage: e.toString()));
    }
  }
}
